use std::time::Instant;

mod commands;

fn main() {
    println!("Test executeing");
    let start_time = Instant::now();
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

    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    println!(
        "execute usetime:{:?}\n transaction: {}",
        execution_time,
        transaction.unwrap()
    );

    println!("\n\nTest deploying");
    let start_time = Instant::now();
    let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
    let program_id = "credlancer_rating_survey_v3.aleo";
    let path = "/Users/huweixiong/code/superfreelancer/ABCDE_ZKHack/Credlancer_Update/aleo_contracts/credlancer_rating_survey/build/";
    let record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}";
    let fee = None;
    let query = None;

    let transaction = commands::deploy(private_key, program_id, path, record, fee, query);
    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    println!(
        "deploy usetime{:?}\n transaction: {}",
        execution_time,
        transaction.unwrap()
    );

    println!("\n\nTest transfering");
    let start_time = Instant::now();
    let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
    let recipient = "aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz";
    let amount = 100000u64;
    let function = "private_to_public";
    let input_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}";
    let fee_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 4593678u64.private,  _nonce: 2519444983733166664452332514213249618009422832813505241649029942240273681354group.public}";
    let query = None;

    let transaction = commands::transfer(
        private_key,
        recipient,
        amount,
        function,
        input_record,
        fee_record,
        None,
        query,
    );
    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    println!(
        "transfer usetime:{:?}\n transaction: {}",
        execution_time,
        transaction.unwrap()
    );

    println!("\n\nTest joining");
    let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
    let first_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 4593678u64.private,  _nonce: 2519444983733166664452332514213249618009422832813505241649029942240273681354group.public}";
    let second_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}";
    let fee_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 2417370u64.private,  _nonce: 6125567204338138114699824935057483504900942792237093903423312221865875650403group.public}";

    let transaction = commands::join(
        private_key,
        first_record,
        second_record,
        fee_record,
        None,
        None,
    );
    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    println!("join usetime:{:?} transaction: {}", execution_time, transaction.unwrap());
}

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
        let path = "/Users/huweixiong/code/superfreelancer/ABCDE_ZKHack/Credlancer_Update/aleo_contracts/credlancer_rating_survey/build/";
        let record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}";
        let fee = None;
        let query = None;

        let transaction = commands::deploy(private_key, program_id, path, record, fee, query);
        println!("deploy transaction: {}", transaction.unwrap());
    }

    #[test]
    fn transfer() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let recipient = "aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz";
        let amount = 100000u64;
        let function = "private_to_public";
        let input_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 4593678u64.private,  _nonce: 2519444983733166664452332514213249618009422832813505241649029942240273681354group.public}";
        let fee_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}";
        let query = None;

        let transaction = commands::transfer(
            private_key,
            recipient,
            amount,
            function,
            input_record,
            fee_record,
            None,
            query,
        );
        println!("transfer transaction: {}", transaction.unwrap());
    }

    #[test]
    fn join() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let first_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 4593678u64.private,  _nonce: 2519444983733166664452332514213249618009422832813505241649029942240273681354group.public}";
        let second_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}";
        let fee_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 2417370u64.private,  _nonce: 6125567204338138114699824935057483504900942792237093903423312221865875650403group.public}";

        let transaction = commands::join(
            private_key,
            first_record,
            second_record,
            fee_record,
            None,
            None,
        );
        println!("join transaction: {}", transaction.unwrap());
    }

    #[test]
    fn split() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 4593678u64.private,  _nonce: 2519444983733166664452332514213249618009422832813505241649029942240273681354group.public}";
        let amount = 1000000u64;
        let transaction = commands::split(
            private_key,
            record,
            amount,
            None,
        );
        println!("join transaction: {}", transaction.unwrap());
    }
}
