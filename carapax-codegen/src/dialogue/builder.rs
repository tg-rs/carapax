use crate::dialogue::meta::DialogueMeta;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn build(meta: DialogueMeta) -> TokenStream {
    let DialogueMeta {
        vis,
        ident,
        ident_inner,
        handler,
        state,
        context,
        input,
        error,
    } = meta;
    quote! {
        #handler
        #[allow(non_camel_case_types)]
        #vis struct #ident;
        #[::carapax::async_trait]
        impl ::carapax::dialogue::DialogueHandler<#context, #state> for #ident {
            type Input = #input;
            type Error = #error;
            async fn handle(
                &mut self,
                state: #state,
                context: &#context,
                input: Self::Input
            ) -> ::std::result::Result<::carapax::dialogue::DialogueResult<#state>, Self::Error> {
                #ident_inner(state, context, input).await
            }
        }
    }
}
