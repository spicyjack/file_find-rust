// fs.rs
use std::env; // env::home_dir()
use std::fs; // fs::read_dir()
//use std::path::Path;
use std::path::PathBuf;

fn main() {
   //let mut idg_dir_path = PathBuf::from("~/Files/Doom/idgames");
   // create an empty PathBuf
   let mut idg_dir = PathBuf::new();
   // call `home_dir()` and act on what it returns
   match env::home_dir() {
      //Some(idg_dir) => println!("{}", idg_dir.display()),
      // home_dir() returned something
      Some(home_dir) => idg_dir = home_dir,
      // home_dir() returned nothing
      None => println!("Can't obtain home directory path!"),
   }

   // append more directories on to idg_dir
   idg_dir.push("Files");
   idg_dir.push("Doom");
   idg_dir.push("idgames");

   // try and read the contents of idg_dir/
   match fs::read_dir(idg_dir) {
      // nope, can't read for some reason
      Err(why) => println!("! {:?}", why.kind()),
      // yep, can read contents of idg_dir, see what items are there
      Ok(items) => for item in items {
         println!("- {:?}", item.unwrap().path());
      }
   }
}
