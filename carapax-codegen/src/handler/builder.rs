use crate::handler::{
    args::{HandlerArgs, HandlerKind},
    meta::HandlerMeta,
};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn build(meta: HandlerMeta, args: Option<HandlerArgs>) -> TokenStream {
    let HandlerMeta {
        vis,
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
                        Err(err) => return ::carapax::HandlerResult::error(err)
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
        #vis struct #ident;
        #[::carapax::async_trait]
        impl ::carapax::Handler<#context> for #ident {
            type Input = #input;
            type Output = #output;
            async fn handle(
                &mut self,
                context: &#context,
                input: Self::Input
            ) -> Self::Output {
                #inner_call
            }
        }
    }
}
