#[derive(Debug, Default)]
pub struct EcpayResultVO {
    pub is_match: i32,
    pub card4no: Option<String>,
    pub card6no: Option<String>,
    pub customfield1: Option<String>,
    pub customfield2: Option<String>,
    pub gwsr: Option<String>,
    pub merchantid: Option<String>,
    pub merchanttradeno: Option<String>,
    pub paymentdate: Option<String>,
    pub rtncode: Option<String>,
    pub rtnmsg: Option<String>,
    pub storeid: Option<String>,
    pub tradeamt: Option<String>,
    pub tradeno: Option<String>,
}
