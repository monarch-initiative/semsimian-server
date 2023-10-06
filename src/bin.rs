// This is a simple web server that uses the semsimian crate for semantic similarity operations.

#[macro_use]
extern crate rocket;

use semsimian::RustSemsimian;
use semsimian_server::{compare_termsets, say_hello, search, ApiConfig};

// In a real application, this would likely be more complex.


#[launch]
pub fn rocket() -> _ {
    /*
    todo: this would be the right place to instantiate a RustSemsimian instance,
          it should get created, then needs to have a termset compare called, then
          be cloned as immutable and stored either in a global variable (meh) or as
          managed shared data by rocket (better?)
    */

    let rss: RustSemsimian = semsimian_server::get_rss_instance();
    let terms1 = vec!["MP:0010771".to_string()];
    let terms2 = vec!["HP:0004325".to_string()];
    rss.termset_pairwise_similarity(&terms1.iter().cloned().collect(), &terms2.iter().cloned().collect());

    //make a static clone of rss
    let rss_clone = rss.clone();


    let config = ApiConfig {
        thing: "hello".to_string(),
        rss: rss_clone
    };

    compare_termsets("MP:0010771".to_string(), "HP:0004325".to_string());
    rocket::build().manage(config).mount("/", routes![say_hello, compare_termsets, search])
}
