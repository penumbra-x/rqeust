use super::http2::{HEADERS_PSEUDO_ORDER, HEADER_PRORIORITY, SETTINGS_ORDER};
use super::tls::{ChromeTlsSettings, NEW_CURVES};
use crate::tls::impersonate::{http2::Http2Settings, ImpersonateSettings};
use crate::tls::TlsResult;
use http::{
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, UPGRADE_INSECURE_REQUESTS, USER_AGENT},
    HeaderMap, HeaderValue,
};

pub(crate) fn get_settings() -> TlsResult<ImpersonateSettings> {
    Ok(ImpersonateSettings::builder()
        .tls(
            ChromeTlsSettings::builder()
                .curves(NEW_CURVES)
                .permute_extensions(true)
                .pre_shared_key(true)
                .enable_ech_grease(true)
                .build()
                .try_into()?,
        )
        .http2(
            Http2Settings::builder()
                .initial_stream_window_size(6291456)
                .initial_connection_window_size(15728640)
                .max_header_list_size(262144)
                .header_table_size(65536)
                .enable_push(false)
                .headers_priority(*HEADER_PRORIORITY)
                .headers_pseudo_order(*HEADERS_PSEUDO_ORDER)
                .settings_order(SETTINGS_ORDER.to_vec())
                .build(),
        )
        .headers(Box::new(header_initializer))
        .build())
}

fn header_initializer(headers: &mut HeaderMap) {
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static(
            "\"Chromium\";v=\"124\", \"Google Chrome\";v=\"124\", \"Not-A.Brand\";v=\"99\"",
        ),
    );
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"macOS\""));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("?1"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("same-site"));
    headers.insert("Sec-Fetch-User", HeaderValue::from_static("document"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("navigate"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br, zstd"),
    );
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US;q=1.0"));
}
