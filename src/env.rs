use ipi::anyhow::{anyhow, Result};

pub fn infer<K: AsRef<str>, R>(key: K) -> Result<R>
where
    R: ::core::str::FromStr,
    <R as ::core::str::FromStr>::Err: ::std::error::Error + Send + Sync + 'static,
{
    let key = key.as_ref();

    ::std::env::var(key)
        .map_err(|_| anyhow!("failed to find the environment variable: {}", key))
        .and_then(|e| e.parse().map_err(Into::into))
}
