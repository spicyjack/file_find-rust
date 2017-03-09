// fs.rs
use std::env; // env::home_dir()
use std::fs; // fs::read_dir()
//use std::path::Path;
use std::path::PathBuf;


enum ItemType {
   Dir,
   File,
   Other,
}


struct FoundItem {
   item_type: ItemType,
   item_path: PathBuf,
}


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

/*
// doesn't work, try!() won't handle _Result_ objects
   for entry in try!(fs::read_dir(idg_dir)) {
       let dir = try!(entry);
       println!("{:?}", dir.path());
   }
*/

/*
   // try and read the contents of idg_dir/
   // - doesn't work, try!() won't handle _Result_ objects
   for entry in try!(fs::read_dir(idg_dir)) {
      let dir = try!(entry);
      // 'entry' is still a _Result_, so this is broken
      let metadata = try!(entry.metadata());
      // 'dir' is still a _DirEntry_
      if dir.is_dir() {
         println!("d {:?}", dir.path());
      } else {
         println!("! {:?}", dir.path());
      }
   }
*/

/*
   if let Ok(entries) = fs::read_dir(idg_dir) {
      for entry in entries {
         // 'item' is a Result, unwrap it to get access to Metadata
         if let Ok(entry) = entry {
               if let Ok(metadata) = entry.metadata() {};
               // entry.path() returns a _Result_
               if let Ok(path) = entry.path() {};
               // 'entry' is a _DirEntry_, and doesn't have an 'is_dir()'
               // method
               if entry.is_dir() {
                  println!("d {:?} {:?}",
                     // 'metadata' isn't available here
                     metadata.file_type(),
                     // the compiler doesn't know what 'path' is here
                     path);
            }
         }
      }
   }
*/

/*
   // working example
   println!("Updated іdgames/ path: {}", idg_dir.display());
   // read the contents of idg_dir/
   match fs::read_dir(idg_dir) {
      Err(why) => println!("! {:?}", why.kind()),
      Ok(paths) => for path in paths {
         println!("> {:?}", path.unwrap().path());
		}
	}
*/

	// working example from:
	// https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_name
	if let Ok(entries) = fs::read_dir(idg_dir) {
		for entry in entries {
			if let Ok(entry) = entry {
			// Here, `entry` is a `DirEntry`.
				println!("{:?}", entry.file_name());
			}
		}
	}
}
