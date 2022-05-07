use ipi::anyhow::{anyhow, Error, Result};

pub trait Infer<'a> {
    type GenesisArgs: ?Sized;
    type GenesisResult;

    fn infer() -> Self
    where
        Self: Sized,
    {
        // init logger
        crate::logger::init_once();

        match <Self as Infer<'a>>::try_infer() {
            Ok(this) => this,
            Err(e) => {
                ::log::error!("failed to infer: {e}");
                panic!("failed to infer: {e}");
            }
        }
    }

    fn try_infer() -> Result<Self>
    where
        Self: Sized;

    fn genesis(
        args: <Self as Infer<'a>>::GenesisArgs,
    ) -> Result<<Self as Infer<'a>>::GenesisResult>;
}

pub fn infer<K: AsRef<str>, R>(key: K) -> Result<R>
where
    R: ::core::str::FromStr,
    <R as ::core::str::FromStr>::Err: Into<Error> + Send + Sync + 'static,
{
    let key = key.as_ref();

    ::std::env::var(key)
        .map_err(|_| anyhow!("failed to find the environment variable: {key}"))
        .and_then(|e| e.parse().map_err(Into::into))
}
