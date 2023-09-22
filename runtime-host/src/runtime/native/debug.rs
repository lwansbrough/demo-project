use crate::WizardRuntimeState;

impl crate::debug::Host for WizardRuntimeState<'_> {
    fn log<'life0,'async_trait>(&'life0 mut self,msg:String,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        Box::pin(async move {
            println!("{:?}", msg);
            wasmtime::Result::Ok(())
        })
    }
}
