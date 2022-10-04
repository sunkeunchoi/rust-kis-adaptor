use anyhow::{anyhow, Result};
use kis_api::models::{
    GetDayOrNight200Response, GetDomesticStockListResponse, GetDomesticStockPossibleOrderResponse,
    GetDomesticStockPossibleOrderResponseAllOfOutput, GetDomesticStockPriceListResponse,
    GetDomesticStockPriceResponse, GetDomesticStockPriceResponseAllOfOutput,
    GetOverseasBalanceResponse, GetOverseasStockListResponse, GetOverseasStockPriceListResponse,
    GetOverseasStockPriceResponse,
};
pub trait Validate<T> {
    fn validate(self) -> Result<T>;
}
pub trait Parse<T> {
    fn parse(self) -> Result<T>;
}
impl Validate<GetDomesticStockListResponse> for GetDomesticStockListResponse {
    fn validate(self) -> Result<GetDomesticStockListResponse> {
        match self.rt_cd.as_str() {
            "0" => Ok(self),
            _ => Err(anyhow!("{} {}", self.msg_cd, self.msg1)),
        }
    }
}
impl Validate<GetOverseasStockPriceListResponse> for GetOverseasStockPriceListResponse {
    fn validate(self) -> Result<GetOverseasStockPriceListResponse> {
        match self.rt_cd.as_str() {
            "0" => Ok(self),
            _ => Err(anyhow!("{} {}", self.msg_cd, self.msg1)),
        }
    }
}
impl Validate<GetOverseasBalanceResponse> for GetOverseasBalanceResponse {
    fn validate(self) -> Result<GetOverseasBalanceResponse> {
        match self.rt_cd.as_str() {
            "0" => Ok(self),
            _ => Err(anyhow!("{} {}", self.msg_cd, self.msg1)),
        }
    }
}
impl Parse<std::vec::Vec<kis_api::models::GetDomesticStockPriceListResponseAllOfOutput>>
    for GetDomesticStockPriceListResponse
{
    fn parse(
        self,
    ) -> Result<std::vec::Vec<kis_api::models::GetDomesticStockPriceListResponseAllOfOutput>> {
        let data = self
            .output
            .expect("No output get_doemstic_stock_price_list");
        Ok(data)
    }
}
impl Validate<GetDomesticStockPriceListResponse> for GetDomesticStockPriceListResponse {
    fn validate(self) -> Result<GetDomesticStockPriceListResponse> {
        match self.rt_cd.as_str() {
            "0" => Ok(self),
            _ => Err(anyhow!("{} {}", self.msg_cd, self.msg1)),
        }
    }
}
impl Validate<GetDomesticStockPossibleOrderResponse> for GetDomesticStockPossibleOrderResponse {
    fn validate(self) -> Result<GetDomesticStockPossibleOrderResponse> {
        match self.rt_cd.as_str() {
            "0" => Ok(self),
            _ => Err(anyhow!("{} {}", self.msg_cd, self.msg1)),
        }
    }
}
impl Validate<GetOverseasStockPriceResponse> for GetOverseasStockPriceResponse {
    fn validate(self) -> Result<GetOverseasStockPriceResponse> {
        match self.rt_cd.as_str() {
            "0" => Ok(self),
            _ => Err(anyhow!("{} {}", self.msg_cd, self.msg1)),
        }
    }
}
impl Validate<GetOverseasStockListResponse> for GetOverseasStockListResponse {
    fn validate(self) -> Result<GetOverseasStockListResponse> {
        match self.rt_cd.as_str() {
            "0" => Ok(self),
            _ => Err(anyhow!("{} {}", self.msg_cd, self.msg1)),
        }
    }
}
impl Validate<GetDayOrNight200Response> for GetDayOrNight200Response {
    fn validate(self) -> Result<GetDayOrNight200Response> {
        match self.rt_cd.as_str() {
            "0" => Ok(self),
            _ => Err(anyhow!("{} {}", self.msg_cd, self.msg1)),
        }
    }
}
impl Parse<GetDomesticStockPriceResponseAllOfOutput> for GetDomesticStockPriceResponse {
    fn parse(self) -> Result<GetDomesticStockPriceResponseAllOfOutput> {
        let boxed = self.output.expect("No output get_domestic_stock_price");
        Ok(*boxed)
    }
}
impl Validate<GetDomesticStockPriceResponse> for GetDomesticStockPriceResponse {
    fn validate(self) -> Result<GetDomesticStockPriceResponse> {
        match self.rt_cd.as_str() {
            "0" => Ok(self),
            _ => Err(anyhow!("{} {}", self.msg_cd, self.msg1)),
        }
    }
}

impl Parse<GetDomesticStockPossibleOrderResponseAllOfOutput>
    for GetDomesticStockPossibleOrderResponse
{
    fn parse(self) -> Result<GetDomesticStockPossibleOrderResponseAllOfOutput> {
        let r = self.validate()?;
        let boxed = r
            .output
            .expect("No output get_doemstic_stock_possible_order");
        Ok(*boxed)
    }
}

impl Parse<bool> for GetDayOrNight200Response {
    fn parse(self) -> Result<bool> {
        match self
            .validate()?
            .output
            .expect("Invalid output")
            .psbl_yn
            .as_str()
        {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => Err(anyhow!("get_overseas_dayornight Unkown output")),
        }
    }
}
