mod commands;


#[cfg(test)]
mod tests {
    use crate::commands;

    #[test]
    fn execute() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let program_id = "credlancer_rating_survey_v2.aleo";
        let function = "submit_assessor";
        let inputs = vec!["10000field", "1field", "1u64"];
        let record =  Some("{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}");
        let fee = Some(3000u64);
        let query = Some("https://vm.aleo.org/api");

        let transaction = commands::execute(
            private_key,
            program_id,
            function,
            inputs,
            record,
            fee,
            query,
        );

        println!("execute transaction: {}", transaction.unwrap());
    }

    #[test]
    fn deploy() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let program_id = "credlancer_rating_survey_v3.aleo";
        let path = Some("/Users/huweixiong/code/superfreelancer/ABCDE_ZKHack/Credlancer_Update/aleo_contracts/credlancer_rating_survey/build/".to_string());
        let record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}";
        let fee = None;
        let query = None;

        let transaction = commands::deploy(private_key, program_id, path, record, fee, query);
        println!("deploy transaction: {}", transaction.unwrap());
    }
}