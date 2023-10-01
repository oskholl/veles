mod parser;

fn main() {
    let response_text = match parser::download_transactions() {
        Ok(response_text) => response_text,
        Err(e) => panic!("Could not download transactions {:?}", e)
    };
    let raw_transactions = parser::parse_transactions(response_text);
    println!("{:?}", raw_transactions);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_transactions() {
        let transaction_str = r#"
{"accountStatement":{"info":{"accountId":"2400222222","bankId":"2010","currency":"CZK","iban":"CZ7920100000002400222222","bic":"FIOBCZPPXXX","openingBalance":195.00,"closingBalance":195.01,"dateStart":1340661600000,"dateEnd":1341007200000,"yearList":null,"idList":null,"idFrom":1148734530,"idTo":1149190193,"idLastDownload":1149190192},"transactionList":{"transaction":[{"column22":{"value":1148734530,"name":"ID pohybu","id":22},"column0":{"value":1340661600000,"name":"Datum","id":0},"column1":{"value":1.00,"name":"Objem","id":1},"column14":{"value":"CZK","name":"Měna","id":14},"column2":{"value":"2900233333","name":"Protiúčet","id":2},"column10":{"value":"Pavel, Novák","name":"Název protiúčtu","id":10},"column3":{"value":"2010","name":"Kód banky","id":3},"column12":{"value":"Fio banka, a.s.","name":"Název banky","id":12},"column4":{"value":"0558","name":"KS","id":4},"column5":null,"column6":null,"column7":null,"column16":null,"column8":{"value":"Příjem převodem uvnitř banky","name":"Typ","id":8},"column9":null,"column18":null,"column25":null,"column26":null,"column17":{"value":2105685816,"name":"ID pokynu","id":17}},{"column22":{"value":1148734781,"name":"ID pohybu","id":22},"column0":{"value":1340661600000,"name":"Datum","id":0},"column1":{"value":-1.00,"name":"Objem","id":1},"column14":{"value":"CZK","name":"Měna","id":14},"column2":{"value":"2900233333","name":"Protiúčet","id":2},"column10":null,"column3":{"value":"2010","name":"Kód banky","id":3},"column12":{"value":"Fio banka, a.s.","name":"Název banky","id":12},"column4":{"value":"0558","name":"KS","id":4},"column5":null,"column6":null,"column7":{"value":" ","name":"Uživatelská identifikace","id":7},"column16":null,"column8":{"value":"Platba převodem uvnitř banky","name":"Typ","id":8},"column9":{"value":"Novák, Jan","name":"Provedl","id":9},"column18":null,"column25":{"value":" ","name":"Komentář","id":25},"column26":null,"column17":{"value":2105687343,"name":"ID pokynu","id":17}},{"column22":{"value":1149190193,"name":"ID pohybu","id":22},"column0":{"value":1341007200000,"name":"Datum","id":0},"column1":{"value":0.01,"name":"Objem","id":1},"column14":{"value":"CZK","name":"Měna","id":14},"column2":null,"column10":null,"column3":null,"column12":null,"column4":null,"column5":null,"column6":null,"column7":null,"column16":null,"column8":{"value":"Připsaný úrok","name":"Typ","id":8},"column9":null,"column18":null,"column25":null,"column26":null,"column17":{"value":2107642322,"name":"ID pokynu","id":17}}]}}}
        "#;
        assert_eq!(4, internal_adder(2, 2));
    }
}
