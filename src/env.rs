use ipi::anyhow::{anyhow, Error, Result};

pub trait Infer {
    type GenesisArgs;
    type GenesisResult;

    fn infer() -> Result<Self>
    where
        Self: Sized;

    fn genesis(args: &<Self as Infer>::GenesisArgs) -> Result<<Self as Infer>::GenesisResult>;
}

pub fn infer<K: AsRef<str>, R>(key: K) -> Result<R>
where
    R: ::core::str::FromStr,
    <R as ::core::str::FromStr>::Err: Into<Error> + Send + Sync + 'static,
{
    let key = key.as_ref();

    ::std::env::var(key)
        .map_err(|_| anyhow!("failed to find the environment variable: {}", key))
        .and_then(|e| e.parse().map_err(Into::into))
}
