// fs.rs
use std::env; // env::home_dir()
use std::fs; // fs::read_dir()
//use std::path::Path;
use std::path::PathBuf;

fn main() {
   //let mut idg_dir_path = PathBuf::from("~/Files/Doom/idgames");
   let mut idg_dir = PathBuf::new();
   match env::home_dir() {
      //Some(idg_dir) => println!("{}", idg_dir.display()),
      Some(home_dir) => idg_dir = home_dir,
      None => println!("Can't obtain home directory path!"),
   }

   idg_dir.push("Files");
   idg_dir.push("Doom");
   idg_dir.push("idgames");

   println!("Updated іdgames/ path: {}", idg_dir.display());
   // read the contents of idg_dir/
   match fs::read_dir(idg_dir) {
      Err(why) => println!("! {:?}", why.kind()),
      Ok(paths) => for path in paths {
         println!("> {:?}", path.unwrap().path());
      }
   }
}
