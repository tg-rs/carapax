use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    FnArg, Ident, ItemFn, LitStr, Path, Result as SynResult, ReturnType, Token, Type,
};

pub(super) struct HandlerMeta {
    ident: Ident,
    ident_inner: Ident,
    handler: ItemFn,
    context: Box<Type>,
    input: Box<Type>,
    output: Option<Box<Type>>,
}

impl Parse for HandlerMeta {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut handler = input.parse::<ItemFn>()?;
        let context = match handler.sig.inputs.first() {
            Some(FnArg::Typed(typed)) => match &*typed.ty {
                Type::Reference(reference) => reference.elem.clone(),
                _ => return Err(input.error("context must be a reference")),
            },
            _ => return Err(input.error("first argument must be context type")),
        };
        let input = match handler.sig.inputs.last() {
            Some(FnArg::Typed(arg)) => arg.ty.clone(),
            _ => return Err(input.error("unable to detect handler input")),
        };
        let output = match &handler.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, output) => Some(output.clone()),
        };
        let ident = handler.sig.ident.clone();
        let ident_inner = Ident::new(&format!("__carapax_{}", ident), Span::call_site());
        handler.sig.ident = ident_inner.clone();
        Ok(HandlerMeta {
            handler,
            ident,
            ident_inner,
            context,
            input,
            output,
        })
    }
}

pub(super) struct HandlerArgs {
    kind: HandlerKind,
}

enum HandlerKind {
    Command(Value),
    Predicate(Path),
}

enum Value {
    Str(LitStr),
    Path(Path),
}

impl Parse for Value {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(if input.peek(LitStr) {
            Self::Str(input.parse()?)
        } else {
            Self::Path(input.parse()?)
        })
    }
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Str(value) => value.to_tokens(tokens),
            Self::Path(value) => value.to_tokens(tokens),
        }
    }
}

impl Parse for HandlerArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let arg = input.parse::<Ident>()?;
        let kind = match arg.to_string().as_ref() {
            "command" => {
                input.parse::<Token![=]>()?;
                HandlerKind::Command(input.parse()?)
            }
            "predicate" => {
                input.parse::<Token![=]>()?;
                HandlerKind::Predicate(input.parse()?)
            }
            arg => return Err(input.error(format!("unexpected argument: {}", arg))),
        };
        Ok(HandlerArgs { kind })
    }
}

pub(super) fn build(meta: HandlerMeta, args: Option<HandlerArgs>) -> TokenStream {
    let HandlerMeta {
        ident,
        ident_inner,
        handler,
        context,
        input,
        output,
    } = meta;
    let mut output = match output {
        Some(output) => quote!(#output),
        None => quote!(()),
    };
    let mut inner_call = quote!(#ident_inner(context, input).await);
    if let Some(HandlerArgs { kind }) = args {
        output = quote!(::carapax::HandlerResult);
        match kind {
            HandlerKind::Command(command) => {
                inner_call = quote! {
                    if input.get_name() == #command {
                        #inner_call.into()
                    } else {
                        ::carapax::HandlerResult::Continue
                    }
                };
            }
            HandlerKind::Predicate(predicate) => {
                inner_call = quote! {
                    let result = match #predicate(context, &input).await {
                        Ok(result) => result,
                        Err(err) => return ::carapax::HandlerError::new(err).into()
                    };
                    if result {
                        #inner_call.into()
                    } else {
                        ::carapax::HandlerResult::Continue
                    }
                }
            }
        }
    };
    quote! {
        #handler
        #[allow(non_camel_case_types)]
        struct #ident;
        #[::carapax::async_trait]
        impl ::carapax::Handler<#context> for #ident {
            type Input = #input;
            type Output = #output;
            async fn handle(
                &mut self,
                context: &mut #context,
                input: Self::Input
            ) -> Self::Output {
                #inner_call
            }
        }
    }
}
