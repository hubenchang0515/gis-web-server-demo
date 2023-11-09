use sqlite::Connection;
use sqlite::State;
use sqlite::Value;

pub struct GisSqlite {
    pub file: String,
    pub conn: Connection,
}

impl GisSqlite {
    pub fn new(file:&str) -> GisSqlite{
        GisSqlite{
            file: String::from(file),
            conn: sqlite::open(file).unwrap(),
        }
    }

    pub fn init(&self) -> bool{
        let mut table_exist = false;
        self.conn.iterate(r"SELECT name FROM sqlite_master WHERE type='table' AND name = 'tiles';", 
                            |pairs| {
                                let &(_, value) = pairs.first().unwrap();
                                table_exist = value.unwrap() == "tiles";
                                true
                            }).unwrap();
        if !table_exist {
            self.conn.execute("CREATE TABLE tiles (x INTEGER, y INTEGER, z INTEGER, image BLOB);").unwrap();
        }

        let cols = ["x", "y", "z"];

        for col in cols {
            let mut index_exist = false;
            self.conn.iterate(format!("SELECT name FROM sqlite_master WHERE type='index' AND name = '{}_index';", col), 
                                |pairs| {
                                    for &(_, value) in pairs.iter() {
                                        if value.unwrap() == format!("{}_index", col) {
                                            index_exist = true;
                                            return true;
                                        }
                                    }
                                    true
                                }).unwrap();

            if !index_exist {
                self.conn.execute(format!("CREATE INDEX {}_index ON tiles ({});", col, col)).unwrap();
            }
        }

        table_exist
    }

    pub fn get(&self, x:u64, y:u64, z:u64) -> Vec<u8> {
        let query = "SELECT image FROM tiles WHERE x=? AND y=? AND z=?;";
        let mut state = self.conn.prepare(query).unwrap();
        state.bind(&[(1, x as i64), (2, y as i64), (3, z as i64)][..]).unwrap();
        if let Ok(State::Row) = state.next() {
            state.read::<Vec<u8>, _>("image").unwrap()
        } else {
            Vec::<u8>::new()
        }
    }

    pub fn set(&self, x:u64, y:u64, z:u64, image:&Vec<u8>) {
        let query = "INSERT INTO tiles VALUES (?, ?, ?, ?) ;";
        let mut state = self.conn.prepare(query).unwrap();
        state.bind::<&[(_, Value)]>(&[
            (1, (x as i64).into()), 
            (2, (y as i64).into()), 
            (3, (z as i64).into()), 
            (4, image.clone().into())
        ][..]).unwrap();
        
        while let Err(_) = state.next(){}
    }
}