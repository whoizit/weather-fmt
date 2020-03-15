extern crate accuweather;
extern crate regex;
extern crate strfmt;

use std::{env, collections::HashMap};
use regex::Regex;
use strfmt::strfmt;

fn main() {
// GET WEATHER CONDITIONS
    let client = accuweather::Accuweather::new(
        env::var("ACCU_API_KEY").unwrap(),
        Some(env::var("ACCU_CITY_ID").unwrap().parse::<i32>().unwrap()),
        Some(env::var("ACCU_LANG").unwrap()),
    );

    let conditions = client.get_current_conditions().unwrap();
    let w = &conditions[0];
    //println!("{:#?}", &w);
    
// GET WEATHER FMT
    let fmt = env::var("WEATHER_FMT").unwrap();
    let re = Regex::new(r"\{([^}]*)\}").unwrap();

    let mut v = Vec::new();
    for caps in re.captures_iter(&fmt) {
        v.push(caps.get(1).unwrap().as_str());
    }
    //println!("{:#?}", &v);

// POPULATE VARS FOR STRFMT
    let mut vars = HashMap::new();
    for i in v {
        vars.insert(i, "bob");
    }
    //println!("{:#?}", &vars);

    //println!(strfmt(&fmt, &vars).unwrap());

    //println!("{:#?}", w[0].weather_text);
    //println!(
    //    "{}°({}°), {}, {}%",
    //    w.temperature.metric.value,
    //    w.real_feel_temperature.metric.value,
    //    w.weather_text,
    //    w.relative_humidity
    //);
}
