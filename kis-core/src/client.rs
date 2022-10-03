#![allow(dead_code)]
use crate::config::Config;
use anyhow::{anyhow, Result};

pub async fn create_token(config: &Config) -> Result<String> {
    let token_request = kis_api::models::CreateTokenRequest::new(
        kis_api::models::create_token_request::GrantType::ClientCredentials,
        config.appkey.to_owned(),
        config.appsecret.to_owned(),
    );
    let r = kis_api::apis::o_auth_api::create_token(&config.configuration, token_request).await?;
    Ok(r.access_token)
}
pub async fn delete_token(config: &Config) -> Result<String> {
    let token = config.configuration.bearer_access_token.clone();
    let revoke_request = kis_api::models::DeleteTokenRequest::new(
        config.appkey.to_owned(),
        config.appsecret.to_owned(),
        token.expect("token is not set"),
    );
    let r = kis_api::apis::o_auth_api::delete_token(&config.configuration, revoke_request).await?;
    match r.code {
        200 => Ok(r.message),
        _ => Err(anyhow!("delete_token {} {}", r.code, r.message)),
    }
}
pub async fn create_hashkey(config: &Config, body: serde_json::Value) -> Result<String> {
    let request_body =
        serde_json::to_value(body).expect("failed to serialize hashkey request body");
    let r = kis_api::apis::o_auth_api::create_hashkey(
        &config.configuration,
        &config.appkey,
        &config.appsecret,
        request_body,
    )
    .await?;
    Ok(r.hash)
}

pub async fn get_doemstic_stock_price_list(
    config: &Config,
    ticker: &str,
    interval: crate::types::Interval,
    is_adjusted: bool,
) -> Result<std::vec::Vec<kis_api::models::GetDomesticStockPriceListResponseAllOfOutput>> {
    let fid_org_adj_prc = if is_adjusted { "1" } else { "0" };
    let fid_period_div_code = interval.to_char();
    let r = kis_api::apis::domestic_stock_api::get_domestic_stock_price_list(
        &config.configuration,
        &config.appkey,
        &config.appsecret,
        "FHKST01010400",
        "J",
        ticker,
        fid_period_div_code,
        fid_org_adj_prc,
    )
    .await?;
    match r.rt_cd.as_str() {
        "0" => Ok(r.output.expect("No output get_doemstic_stock_price_list")),
        _ => Err(anyhow!(
            "get_doemstic_stock_price_list {} {}",
            r.msg_cd,
            r.msg1
        )),
    }
}
pub async fn get_overseas_balance(
    config: &Config,
) -> Result<kis_api::models::GetOverseasBalanceResponse> {
    let tr_id = if config.is_paper {
        "VTRP6504R"
    } else {
        "CTRP6504R"
    };
    let r = kis_api::apis::overseas_stock_api::get_overseas_balance(
        &config.configuration,
        &config.appkey,
        &config.appsecret,
        tr_id,
        &config.cano,
        &config.acnt_prdt_cd,
        "01",
        "000",
        "00",
        "01",
        Some("P"),
    )
    .await?;
    match r.rt_cd.as_str() {
        "0" => Ok(r),
        _ => Err(anyhow!("get_overseas_balance {} {}", r.msg_cd, r.msg1)),
    }
}
pub async fn get_overseas_stock_price_list(
    config: &Config,
    exchange: crate::types::Exchange,
    ticker: &str,
    interval: crate::types::Interval,
    is_adjusted: bool,
) -> Result<kis_api::models::GetOverseasStockPriceListResponse> {
    let modp = if is_adjusted { "1" } else { "0" };
    let r = kis_api::apis::overseas_stock_api::get_overseas_stock_price_list(
        &config.configuration,
        &config.appkey,
        &config.appsecret,
        "HHDFS76240000",
        "",
        exchange.to_string().as_str(),
        ticker,
        interval.to_num_str(),
        "",
        modp,
    )
    .await?;
    match r.rt_cd.as_str() {
        "0" => Ok(r),
        _ => Err(anyhow!(
            "get_overseas_stock_price_list {} {}",
            r.msg_cd,
            r.msg1
        )),
    }
}
pub async fn get_overseas_stock_price(
    config: &Config,
    exchange: crate::types::Exchange,
    ticker: &str,
) -> Result<kis_api::models::GetOverseasStockPriceResponse> {
    let r = kis_api::apis::overseas_stock_api::get_overseas_stock_price(
        &config.configuration,
        &config.appkey,
        &config.appsecret,
        "HHDFS00000300",
        "",
        exchange.to_string().as_str(),
        ticker,
    )
    .await?;
    match r.rt_cd.as_str() {
        "0" => Ok(r),
        _ => Err(anyhow!("get_overseas_stock_price {} {}", r.msg_cd, r.msg1)),
    }
}
pub async fn get_overseas_dayornight(config: &Config) -> Result<bool> {
    let r = kis_api::apis::overseas_stock_api::get_day_or_night(
        &config.configuration,
        &config.appkey,
        &config.appsecret,
        "JTTT3010R",
    )
    .await?;
    match r.rt_cd.as_str() {
        "0" => match r.output.expect("Invalid output").psbl_yn.as_str() {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => Err(anyhow!("get_overseas_dayornight Unkown output")),
        },
        _ => Err(anyhow!("get_overseas_dayornight {} {}", r.msg_cd, r.msg1)),
    }
}
#[allow(clippy::collapsible_else_if)]
pub async fn get_overseas_stock_list(
    config: &Config,
    market: crate::types::Market,
    is_night: bool,
) -> Result<kis_api::models::GetOverseasStockListResponse> {
    let tr_id = if config.is_paper {
        if is_night {
            "VTTT3012R"
        } else {
            "VTTS3012R"
        }
    } else {
        if is_night {
            "JTTT3012R"
        } else {
            "TTTS3012R"
        }
    };
    let ovrs_excg_cd = market.to_string();
    let tr_crcy_cd = market.get_currency();
    let r = kis_api::apis::overseas_stock_api::get_overseas_stock_list(
        &config.configuration,
        &config.appkey,
        &config.appsecret,
        tr_id,
        &config.cano,
        &config.acnt_prdt_cd,
        &ovrs_excg_cd,
        tr_crcy_cd,
        "",
        "",
        Some("P"),
    )
    .await?;
    match r.rt_cd.as_str() {
        "0" => Ok(r),
        _ => Err(anyhow!("get_overseas_stock_list {} {}", r.msg_cd, r.msg1)),
    }
}
pub async fn get_doemstic_stock_possible_order(
    config: &Config,
    ticker: &str,
    price: u32,
    cma: bool,
    overseas: bool,
) -> Result<kis_api::models::GetDomesticStockPossibleOrderResponseAllOfOutput> {
    let tr_id = if config.is_paper {
        "VTTC8908R"
    } else {
        "TTTC8908R"
    };
    let ord_unpr = format!("{}", price);
    let ord_dvsn = "00";
    let cma_evlu_amt_icld_yn = if cma { "Y" } else { "N" };
    let ovrs_icld_yn = if overseas { "Y" } else { "N" };
    let r = kis_api::apis::domestic_stock_api::get_domestic_stock_possible_order(
        &config.configuration,
        &config.appkey,
        &config.appsecret,
        tr_id,
        &config.cano,
        &config.acnt_prdt_cd, // Not required
        ticker,
        &ord_unpr,
        ord_dvsn,
        cma_evlu_amt_icld_yn,
        ovrs_icld_yn,
        Some("P"),
    )
    .await?;
    match r.rt_cd.as_str() {
        "0" => {
            let boxed = r
                .output
                .expect("No output get_doemstic_stock_possible_order");
            Ok(*boxed)
        }
        _ => panic!("{} {}", r.msg_cd, r.msg1),
    }
}
pub async fn get_domestic_stock_price(
    config: &Config,
    ticker: &str,
) -> Result<kis_api::models::GetDomesticStockPriceResponseAllOfOutput> {
    let r = kis_api::apis::domestic_stock_api::get_domestic_stock_price(
        &config.configuration,
        &config.appkey,
        &config.appsecret,
        "FHKST01010100",
        "J",
        ticker,
    )
    .await?;
    match r.rt_cd.as_str() {
        "0" => {
            let boxed = r.output.expect("No output get_domestic_stock_price");
            Ok(*boxed)
        }
        _ => Err(anyhow!("get_domestic_stock_price {} {}", r.msg_cd, r.msg1)),
    }
}
pub async fn get_domestic_stock_list(
    config: &Config,
    group_by_ticker: bool,
    include_funds: bool,
    include_history: bool,
) -> Result<kis_api::models::GetDomesticStockListResponse> {
    let tr_id = if config.is_paper {
        "VTTC8434R"
    } else {
        "TTTC8434R"
    };
    let prcs_dvsn = if include_history { "00" } else { "01" };
    let fund_sttl_iclud_yn = if include_funds { "Y" } else { "N" };
    let inqr_dvsn = if group_by_ticker { "02" } else { "01" };
    let r = kis_api::apis::domestic_stock_api::get_domestic_stock_list(
        &config.configuration,
        &config.appkey,
        &config.appsecret,
        tr_id,
        &config.cano,
        &config.acnt_prdt_cd,
        "N",
        "",
        inqr_dvsn,
        "01",
        fund_sttl_iclud_yn,
        "N",
        prcs_dvsn,
        "",
        "",
        Some("P"),
    )
    .await?;
    match r.rt_cd.as_str() {
        "0" => Ok(r),
        _ => Err(anyhow!("get_domestic_stock_list {} {}", r.msg_cd, r.msg1)),
    }
}
