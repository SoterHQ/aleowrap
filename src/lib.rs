pub mod commands;
pub use commands::*;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use snarkvm::prelude::{transaction::Transaction, Locator, Testnet3};

    use crate::commands;

    #[test]
    fn execute() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let program_id = "credlancer_rating_survey_v2.aleo";
        let function = "submit_assessor";
        let inputs = vec![
            "10000field".to_string(),
            "1field".to_string(),
            "1u64".to_string(),
        ];
        let record =  Some("{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}");
        let fee = Some(3000u64);
        let query = Some("https://api.explorer.aleo.org/v1");

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
            Some(input_record),
            Some(fee_record),
            None,
            query,
        );
        println!("transfer transaction: {}", transaction.unwrap());
    }

    #[test]
    fn join() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let first_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}";
        let second_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 4593678u64.private,  _nonce: 2519444983733166664452332514213249618009422832813505241649029942240273681354group.public}";
        let fee_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 2417370u64.private,  _nonce: 6125567204338138114699824935057483504900942792237093903423312221865875650403group.public}";

        let transaction = commands::join(
            private_key,
            first_record,
            second_record,
            Some(fee_record),
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
        let transaction = commands::split(private_key, record, amount, None);
        println!("join transaction: {}", transaction.unwrap());
    }

    fn handle_transaction() {
        let broadcast = Some("https://api.explorer.aleo.org/v1/testnet3/transaction/broadcast".to_string());
        let dry_run = true;
        let transaction = Transaction::<Testnet3>::from_str("{\"type\":\"execute\",\"id\":\"at1uva8sflf04zv9xrhj80m0235t6xjze3sy8k03gc4jkcenvyfevqsepnrz6\",\"execution\":{\"transitions\":[{\"id\":\"as1txhjav9wndkq9pfn4nykkrcg4lxfxwq24rxt4wvr3g2u3lpyqcyqkdz9zu\",\"program\":\"credits.aleo\",\"function\":\"join\",\"inputs\":[{\"type\":\"record\",\"id\":\"8207379068955116269639175434994688421996473700583520262279895765666551203827field\",\"tag\":\"1602157761220171459679818852257387973920143610173354158982011320943568886941field\"},{\"type\":\"record\",\"id\":\"5221390751173157670415977516208693778225399959346072358354949117155433388889field\",\"tag\":\"5681216027952910951278873606706974204380422110171414709160327511571292689732field\"}],\"outputs\":[{\"type\":\"record\",\"id\":\"5295094982758899673983099000031374627924176263561570489106973476672670660850field\",\"checksum\":\"7316258972270638828118136865643848469974050735165962510759761735283799316418field\",\"value\":\"record1qyqsqkdenfvzx9y2z3dr742z6ppxascq0yqjyjqc20yyughnps4l6xqgqyxx66trwfhkxun9v35hguerqqpqzqze2wm5ar23dygj98yx7wantgjzgnu8rvpdv39xtx444fyluuxypsc7rpk8xmzytmsc7jgs7dpqfl0kh8m494qfvrzsvxs9qx9f4sqszlwtrtc\"}],\"tpk\":\"895196498109715282937001323989404548642801744113121286707183793547030944230group\",\"tcm\":\"3347398143045159444515917718388449219161331631409270136747926269611065077200field\"}],\"global_state_root\":\"ar1k2dt9te92nae7tses8x3c0d6mdu9twk38djl5kv23ymsrfnlh5zs2qzvu8\",\"proof\":\"proof1qqpqqqqqqqqqqqqzqqqqqqqqqqqqzqqqqqqqqqqq09n3zs2tk5qypk9rqsuf0wjhfdu0nw376qeeg37ljs3dv9jud7qmek63ejjk2dp5xf6kg0pgwkuqq8lsp2crmhlcytdche5kf4q542yfju44t8y563cu5dz0zzsf3tzyarn83hsauvgflvm8w9ljqfygqr923yxketkf3mxypeqrk3lzzmt9dyh7n685ddefgf22vnvxrgvkgwh0kjxljtdq6zu4p7j2nutd8qqsft26czwtt8fllnlqz2ceehqzrhe9lqccxs5cjxsgkulqp3lyxudqlcp33wfecm72ztuhky5yzwquvr0zlh64fmay83cm3sqjfuuy89ddxfl0xrj6jxhd9e8ndksce7tmwvqfkc5ewu7zp7kqgqtc565q8vf2gq5vzq5z67esrktu2u78hzpcpj246lck03xmewcy8d84gg6r625scvtvcdw6q2u6fuqn3thgqjef6f42fwlqeu32m0cr68ctujqhn4whpdyd8v9xu06t4lhavtxlha2ax6kdtnqgn62cley4wwwksqrfmwvvc8cx2l87j57hkqsv2lfpecyv5lt3qmw9rp8axknvaynzt78n9hedhdh3qklelpxwkxw20qzzp2ppn4drpqqrlt05vgjyy8cs9s49vs4etdv09ksl2u88pqul9rdqv30yqcv436lavgya46g507qszwt4pa0j9nfv0d52rtzgrrwm6f5gffuj9c45p29h2esec6astpflrhjhl6z283fcsmzvzlan9fsnsyry2gzdtxw5np465x9mjsadl6trajv68v826gallt6jcrlzc4hkzvxzdg2suwkn29amvwpu0624lq8cz3uascegz73ckjpmzervykcphlv97fpplfkpz8fwrh57emy9njhpxslqpfeyuqmc74t8ak780wqmevw8pp9ty8tz5hvj3mk2wdzx8zy3yh6hk99u0qnxydvfm74a2tt464q24f5c644yxvw35slxxtcqrlk9xjudhf7d7pyg66wr3dagpm90ceten39fcpsu9eg0pypl38vj06v9krkgw48ew5pp74789nwcq3w0d7x9uqpaqcrw937r3tnrzsec8graj5vwwp0wpvnt6vmervlruwzef043pnhp6l8r89x978z3sq0s6zc0tvjjjqza5ztpuf5hwga25yahfhjmnuchrmqn629z8e2hhhece43vyuzsvmzxa5zk2e6kuqyl2vr6qcndmh8sx0wqwqckhz30cdtvxny6l4l3fhz3s78dyngsejqmy0zu24326ga3gcfes9kf96q94drgny72fkqpkh9gwapxf394rvuf5vjkue8jl85xmsy4cjkdtu6zgqmxu4mc75jh45wd7cxp54cp7yqxtx5yh7n8meupqz878uxzruepp2j63nxcns24yjqahwudl8s3yxg62y336l22jd2uzwj2vagqz4wnkyvjhuwmv7z9l96pczg0q7xwwrrxzyql8mknnx9zt974tnsf6jkqsd8mfwxpepjyzj3s4d29ukcwwxfnmqw5txtp8ha360pnl58qpwksef9pd2tq7yqfa9c0frj3wj5kx65zwcad8dy39ertcrw66qxuc7p8y65ztez78dysh20c26g0eqaz0ml23rc5xapuc0tn6qd4ptw5gkkp0pklgc76kw04qh53n3rq8f58myn4jv2gmkl5mg22jrn3qqvman0du5nt70f7tehs7ta73q7el7jj9f0hk5ghcpzrfsjd3vqtnnng3vv4hm6523uys7knq6y8yfw07yc5lgw04deglh95xmw84qrvl9aqh75ecxqup5djaw5ra3nm67apr29jzvr5wj792wrhluqu7zvy67m5s7vfnvutjhje8ya68wgae0an47m9sj2h3zhe2d94mk9yqar3u33gsnh5yxcdnwj42zneyzm8gftyzd6fcqy8fk607dc6l82s7qsqqqqqqqqqqrv5ty52kffk27v0f8dp78ehzv6a3p5ll4523lrf7gzvvtxw203su0ldkl224c3th7kmuse3cm7yprgmh0hxtlljcdwv3ntxk6tapdqv4eekplsh3wjc3kq6detwvm5p5e93j7g5tyk62m837h493w49758j9752fgr465zje836esgm5zje2z3vztqlf4h6mse4wfxr036y5z9w47tcutnr907p3d7ys08nayx7fc6g7w6m2rnmau3zr0dw3x38qfmpvsmjdzu7uq346464hymlkq7g29d5dqtw9py9h7usasqu3x4suqsqqqqqqqqqqqprf5clapa5k8tjc7r75gryvsdpuz3hkkw8z5z5xay657k92yrjjwwwewdhxu7z0rzcg43kk4gqvqqpytuas65ecfgyj5ahytpn8n7nwfhqs4zk3q4juhj2dzacxxdmlgrqp4s9v08dd3n4kn4j6zdc569zcvkz53yfgwg3lz6tjuxf987gfahxjcf7hse5h3308zp98crk2vspqqqqre8vya\"},\"fee\":{\"transition\":{\"id\":\"as15spxhrpe2t7ajv8tjjchv4nqhv87qlm8vg5tregf2rtxxc7nccrqe2ufpq\",\"program\":\"credits.aleo\",\"function\":\"fee\",\"inputs\":[{\"type\":\"record\",\"id\":\"460072901039400048185877372301144744403149777900430199316013966113759038912field\",\"tag\":\"1341937600066340766097115975476493129569306144659225303416507770632567802197field\"},{\"type\":\"public\",\"id\":\"5956047454483565113541842913250656043845387742732802993849947475977433037648field\",\"value\":\"5064u64\"},{\"type\":\"public\",\"id\":\"2110715394209556554439509737922449757491924443017427248934688110088533251521field\",\"value\":\"3027002017637267329684946804739551646226888457248954059532127224664626782152field\"}],\"outputs\":[{\"type\":\"record\",\"id\":\"1630015763875032268979173786968791096739017264930072937445637779735575985777field\",\"checksum\":\"1213515472081352854665547838787575886028464452170841430603666303907140117860field\",\"value\":\"record1qyqsp6ysvecq05kvrw5a28eygr83kar07zwdu9qrj5d2f5f685q926sqqyxx66trwfhkxun9v35hguerqqpqzqxmy6ech6488dfq5sj67meh7zp9rrlft6txmkzdkcurtvtm3yj2quw4mr2v6z8389mry7kxyr9x4nhxg3vx8v4emjq5cd96nxe05hzqcs3w3e4\"}],\"tpk\":\"1423300945299382026862608493668576223119946245590937239632058536935227322155group\",\"tcm\":\"7977282115560850725898298121377221689146380153098270793597564723158118923769field\"},\"global_state_root\":\"ar1r4pp22jeqhshmujh0j7l3pu5ps7s9p24t9eda8j0vu4c2n74mc8qxwme4u\",\"proof\":\"proof1qqpqqqqqqqqqqqqpqqqqqqqqqqqqzqqqqqqqqqqqe58mj9978jszvyy6nt63u946s9rnw5sh85ezaqksxj0klsxk3vdyhg4f8l6f2m4f98aqja2y98lcqt8npn06w6kmmrswgjetxg7f573xx8matfh7rsjlj0eneq8548rccgdc55z7h4c9kvlvu873ze7xsqmt9g3s5yh8yf4g4wqzswvpaxs9frgna9wse4pfqcyzwg7mscdwefwmrvfh6w9wxlj8kfe32yegeqwkhdrr03tvjhl2a7c30efnqdpfsda23h7g22hn0ye2h3al52hqtyldnxgc2cww2hlanpvlenhkzuq73egtj30v6jyw7emluv4qh0y2h8yt0kq9yljfkc07r3zapnylk0juyaagus7kvy5mw3lmx4dm37sq5n049sdzgay7jxagz3ev4js7rcl9l02ws2zx2653zgkyhur8epwfn7q2ys6p3n2zfsjpc2cksj9czqw77935daafte5c9l6cgl9269mjq93wg78yx0y5lxkunzerpcj2dwpj732v99plk6gf68yetc76xvqeg38s9eey70kgcer3rjmcmqcmt4rvkukgd5n850lnxu0wf08he8p9tl96j9xcvpxc8j5hgvpcfpvq3waezw6vjcrw7cmfwp9gc26j097dnc5tl7rhw4h82s86zcqmftr0u352et524eq62j6xp43erh2spd669t32zs4pjpzr5xjuxfvzkwkaslhlc2m2lsdyu89nqnucpmgtyuz5vcmvn8svqldtc5hka50fsq8mlud50ctj2kktg5e9ngzezxhuqcqe8pr08xrc72k2gkjsedd7u7g07xms8unetjk65qe80tnq3qpyws0drnvtm53n3dq6wyjgejtt6efdcxx6eqhvkfygyvpl3skazxe4vrwgdyu6gnsx734uc8xa2zqlmp944vz6qh566kwh84ft24egpjya4w4xqqxkucpvcw84me89vx7u2ruyl8ct3dnx8r7xats35zcqmezmzswq46secws95fjshaj0xz9wlyrt4zwc3df5u9csuah7yejxuhth085nanlznpph7cdw0vmsqwd894v04rw9fzlfpvulzmfnwpqzwtqn9jqa08vlcsq293f7hgs34lkf9tkvp9svexd80jvcedxwqqknk7ntkpc6djy5h2u3mfh3rapmg8329jev46xt06kxxwpnxanynu0f0hytuera7lfyh34adkxynq20fldgcxvpwwgdhh89y9pv3tlcpd0t82wh9zkrw6jyl23a4f3wzrh4q90c5rrkhvqpp86p0s74smfml5dhfcffy80jy5jauvus7unq2g08q5e36q8xwxf5n0x2v3hnpkyq20aqr0l2ruwwkxqqr3svr9gq5c8atfem08e6j4wdk3swhezv3j30lc360e8zklhphuuapsh3ec8el2k5mrmsn4z38cta5yk7z5auad7dvlals7jg5vwqt3ruwufs2pgrxk7jk3sayga785gj05p8c06l0290ycc9ucazu2rrkjemyem2pwx37q5tnyx82scxtxvtf8s08th5z8g5069j47v7ad4mv0cfc48sytudrkj4aeykl75pwgz23turgue38th0a6h9wjyfkjd092u3t3qx5cs5p4uu2ep7y3nj3e32aepdvquylaya4flrva3wuunx3lxgv5rsyqqqqqqqqqqqtefkfrfeeqp3rl26hyg00cfh0r6p94pwkkasx3lvndkr2qcr3qraw3v2ek7wwklpekc9zl00uzge5cvpqfefd8a7vgfm7044g2r46q5k27v84sq0zk52r70jguln9803hs7xfr4a2ywpzrvhdv3z0a0lqlhzxdhurnp8amwuulmuk0q69kx0yx642s3v5ymdumjkxygauyjq6ed4vn3x2cdrxgldwfl2d4xrx42frvrl6sea3p6gczx8ffe7q7cvg007rutwllchllvy8xsq7vexyaz7qkl3q8rwvrmvq22w2gkgfvgqyqqqqqqqqqqqt4veavv3he8ld5429r8697ljlqv3xsx8shlegznj3g5wvdj6ju0j59emnxjdtq8dnpdrt4s8jmxgqqvcyt8zj7hdva0en5pqxjjphynyfwf5s46jv5dt0rtgtvvmcdmwqpmvlpn9wghyshdz3jf7v928amflydzruxpq8re4u9yvdhwzlp7ddxagazqahdr9xdq2zn8rnhtgeqqqqq9ygh22\"}}").unwrap();
        let operation = "".to_string();
        let program_id = "credits.aleo";
        let function = "join";

        let locator =
            Locator::<Testnet3>::from_str(&format!("{}/{}", program_id, function)).unwrap();
        // pub fn handle_transaction(broadcast: Option<String>, dry_run: bool, store: Option<String>, transaction: transaction<CurrentNetwork>, operation: String) -> Result<String>
        let handle_res =
            commands::Command::handle_transaction(broadcast, dry_run, None, transaction, operation);
    }
}
