use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct DiscoveryDatabase {
    pub guild_id: String,
    pub guild_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgoliaResponse {
    pub hits: Vec<Hit>,
    pub nb_hits: i64,
    pub offset: i64,
    pub length: i64,
    pub exhaustive_nb_hits: bool,
    pub exhaustive_typo: bool,
    pub exhaustive: Exhaustive,
    pub query: String,
    pub params: String,
    #[serde(rename = "processingTimeMS")]
    pub processing_time_ms: i64,
    #[serde(rename = "processingTimingsMS")]
    pub processing_timings_ms: ProcessingTimingsMs,
    #[serde(rename = "serverTimeMS")]
    pub server_time_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub banner: Option<String>,
    #[serde(rename = "approximate_presence_count")]
    pub approximate_presence_count: i64,
    #[serde(rename = "approximate_member_count")]
    pub approximate_member_count: i64,
    #[serde(rename = "premium_subscription_count")]
    pub premium_subscription_count: i64,
    #[serde(rename = "preferred_locale")]
    pub preferred_locale: String,
    #[serde(rename = "auto_removed")]
    pub auto_removed: bool,
    #[serde(rename = "discovery_splash")]
    pub discovery_splash: Option<String>,
    #[serde(rename = "primary_category_id")]
    pub primary_category_id: i64,
    #[serde(rename = "vanity_url_code")]
    pub vanity_url_code: Option<String>,
    #[serde(rename = "is_published")]
    pub is_published: bool,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub features: Vec<String>,
    pub categories: Vec<Category>,
    #[serde(rename = "primary_category")]
    pub primary_category: Option<PrimaryCategory>,
    #[serde(rename = "objectID")]
    pub object_id: String,
    #[serde(rename = "_highlightResult")]
    pub highlight_result: HighlightResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    #[serde(rename = "is_primary")]
    pub is_primary: bool,
    pub name: String,
    #[serde(rename = "name_localizations")]
    pub name_localizations: Option<NameLocalizations>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NameLocalizations {
    pub bg: Option<String>,
    pub cs: Option<String>,
    pub da: Option<String>,
    pub de: String,
    pub el: Option<String>,
    pub fi: Option<String>,
    pub fr: String,
    pub hi: Option<String>,
    pub hr: Option<String>,
    pub hu: Option<String>,
    pub it: Option<String>,
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub lt: Option<String>,
    pub nl: Option<String>,
    pub no: Option<String>,
    pub pl: Option<String>,
    pub ro: Option<String>,
    pub ru: String,
    pub th: Option<String>,
    pub tr: Option<String>,
    pub uk: Option<String>,
    pub vi: Option<String>,
    #[serde(rename = "en-GB")]
    pub en_gb: Option<String>,
    #[serde(rename = "en-US")]
    pub en_us: Option<String>,
    #[serde(rename = "es-ES")]
    pub es_es: Option<String>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<String>,
    #[serde(rename = "sv-SE")]
    pub sv_se: Option<String>,
    #[serde(rename = "zh-CN")]
    pub zh_cn: Option<String>,
    #[serde(rename = "zh-TW")]
    pub zh_tw: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryCategory {
    pub id: i64,
    #[serde(rename = "is_primary")]
    pub is_primary: bool,
    pub name: String,
    #[serde(rename = "name_localizations")]
    pub name_localizations: Option<NameLocalizations2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NameLocalizations2 {
    pub de: String,
    pub fr: String,
    pub ru: String,
    pub bg: Option<String>,
    pub cs: Option<String>,
    pub da: Option<String>,
    pub el: Option<String>,
    pub fi: Option<String>,
    pub hi: Option<String>,
    pub hr: Option<String>,
    pub hu: Option<String>,
    pub it: Option<String>,
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub lt: Option<String>,
    pub nl: Option<String>,
    pub no: Option<String>,
    pub pl: Option<String>,
    pub ro: Option<String>,
    pub th: Option<String>,
    pub tr: Option<String>,
    pub uk: Option<String>,
    pub vi: Option<String>,
    #[serde(rename = "en-GB")]
    pub en_gb: Option<String>,
    #[serde(rename = "en-US")]
    pub en_us: Option<String>,
    #[serde(rename = "es-ES")]
    pub es_es: Option<String>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<String>,
    #[serde(rename = "sv-SE")]
    pub sv_se: Option<String>,
    #[serde(rename = "zh-CN")]
    pub zh_cn: Option<String>,
    #[serde(rename = "zh-TW")]
    pub zh_tw: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightResult {
    pub name: Name,
    pub description: Description,
    #[serde(default)]
    pub keywords: Vec<Keyword>,
    #[serde(default)]
    pub categories: Vec<Category2>,
    #[serde(rename = "primary_category")]
    pub primary_category: Option<PrimaryCategory2>,
    #[serde(rename = "vanity_url_code")]
    pub vanity_url_code: Option<VanityUrlCode>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category2 {
    #[serde(rename = "name_localizations")]
    pub name_localizations: NameLocalizations3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NameLocalizations3 {
    pub bg: Option<Bg>,
    pub cs: Option<Cs>,
    pub da: Option<Da>,
    pub de: De,
    pub el: Option<El>,
    pub fi: Option<Fi>,
    pub fr: Fr,
    pub hi: Option<Hi>,
    pub hr: Option<Hr>,
    pub hu: Option<Hu>,
    pub it: Option<It>,
    pub ja: Option<Ja>,
    pub ko: Option<Ko>,
    pub lt: Option<Lt>,
    pub nl: Option<Nl>,
    pub no: Option<No>,
    pub pl: Option<Pl>,
    pub ro: Option<Ro>,
    pub ru: Ru,
    pub th: Option<Th>,
    pub tr: Option<Tr>,
    pub uk: Option<Uk>,
    pub vi: Option<Vi>,
    #[serde(rename = "en-GB")]
    pub en_gb: Option<EnGb>,
    #[serde(rename = "en-US")]
    pub en_us: Option<EnUs>,
    #[serde(rename = "es-ES")]
    pub es_es: Option<EsEs>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<PtBr>,
    #[serde(rename = "sv-SE")]
    pub sv_se: Option<SvSe>,
    #[serde(rename = "zh-CN")]
    pub zh_cn: Option<ZhCn>,
    #[serde(rename = "zh-TW")]
    pub zh_tw: Option<ZhTw>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bg {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cs {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Da {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct De {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct El {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fi {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fr {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hi {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hr {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hu {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct It {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ja {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ko {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lt {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nl {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct No {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pl {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ro {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ru {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Th {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tr {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uk {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vi {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnGb {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnUs {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsEs {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtBr {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvSe {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZhCn {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZhTw {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryCategory2 {
    #[serde(rename = "name_localizations")]
    pub name_localizations: NameLocalizations4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NameLocalizations4 {
    pub de: De2,
    pub fr: Fr2,
    pub ru: Ru2,
    pub bg: Option<Bg2>,
    pub cs: Option<Cs2>,
    pub da: Option<Da2>,
    pub el: Option<El2>,
    pub fi: Option<Fi2>,
    pub hi: Option<Hi2>,
    pub hr: Option<Hr2>,
    pub hu: Option<Hu2>,
    pub it: Option<It2>,
    pub ja: Option<Ja2>,
    pub ko: Option<Ko2>,
    pub lt: Option<Lt2>,
    pub nl: Option<Nl2>,
    pub no: Option<No2>,
    pub pl: Option<Pl2>,
    pub ro: Option<Ro2>,
    pub th: Option<Th2>,
    pub tr: Option<Tr2>,
    pub uk: Option<Uk2>,
    pub vi: Option<Vi2>,
    #[serde(rename = "en-GB")]
    pub en_gb: Option<EnGb2>,
    #[serde(rename = "en-US")]
    pub en_us: Option<EnUs2>,
    #[serde(rename = "es-ES")]
    pub es_es: Option<EsEs2>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<PtBr2>,
    #[serde(rename = "sv-SE")]
    pub sv_se: Option<SvSe2>,
    #[serde(rename = "zh-CN")]
    pub zh_cn: Option<ZhCn2>,
    #[serde(rename = "zh-TW")]
    pub zh_tw: Option<ZhTw2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct De2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fr2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ru2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bg2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cs2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Da2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct El2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fi2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hi2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hr2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hu2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct It2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ja2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ko2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lt2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nl2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct No2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pl2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ro2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Th2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tr2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uk2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vi2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnGb2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnUs2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsEs2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtBr2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvSe2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZhCn2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZhTw2 {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VanityUrlCode {
    pub value: String,
    pub match_level: String,
    pub matched_words: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exhaustive {
    pub nb_hits: bool,
    pub typo: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingTimingsMs {
    #[serde(rename = "_request")]
    pub request: Request,
    pub after_fetch: AfterFetch,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub round_trip: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AfterFetch {
    pub format: Format,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub highlighting: i64,
    pub total: i64,
}
