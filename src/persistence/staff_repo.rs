use crate::models::staff::Staff;
use rusqlite::{Connection, Error, Statement, named_params};

pub struct StaffRepo<'a> {
    conn: &'a Connection,
    insert_stmt: Option<Statement<'a>>,
    fetch_by_id_stmt: Option<Statement<'a>>,
    fetch_all_stmt: Option<Statement<'a>>,
}

impl<'a> StaffRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        StaffRepo {
            conn,
            insert_stmt: None,
            fetch_by_id_stmt: None,
            fetch_all_stmt: None,
        }
    }

    pub fn insert(&mut self, staff: &Staff) -> Result<i64, Error> {
        if self.insert_stmt.is_none() {
            self.insert_stmt = Some(self.conn.prepare(
                "
                INSERT INTO staff (staff_first, staff_last, staff_email, staff_title)
                VALUES (:first, :last, :email, :title)
            ",
            )?);
        }
        self.insert_stmt.as_mut().unwrap().execute(named_params! {
            ":first": staff.staff_first,
            ":last":  staff.staff_last,
            ":email": staff.staff_email,
            ":title": staff.staff_title,
        })?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn fetch_by_id(&mut self, id: i64) -> Result<Staff, Error> {
        if self.fetch_by_id_stmt.is_none() {
            self.fetch_by_id_stmt = Some(self.conn.prepare(
                "
                SELECT staff_id, staff_first, staff_last, staff_email, staff_title, created_at
                FROM staff WHERE staff_id = :id
            ",
            )?);
        }
        self.fetch_by_id_stmt
            .as_mut()
            .unwrap()
            .query_row(named_params! { ":id": id }, |row| {
                Ok(Staff {
                    staff_id: row.get(0)?,
                    staff_first: row.get(1)?,
                    staff_last: row.get(2)?,
                    staff_email: row.get(3)?,
                    staff_title: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })
    }

    pub fn fetch_all(&mut self) -> Result<Vec<Staff>, Error> {
        if self.fetch_all_stmt.is_none() {
            self.fetch_all_stmt = Some(self.conn.prepare(
                "
                SELECT staff_id, staff_first, staff_last, staff_email, staff_title, created_at
                FROM staff ORDER BY staff_last, staff_first
            ",
            )?);
        }
        let mut rows = self.fetch_all_stmt.as_mut().unwrap().query([])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(Staff {
                staff_id: row.get(0)?,
                staff_first: row.get(1)?,
                staff_last: row.get(2)?,
                staff_email: row.get(3)?,
                staff_title: row.get(4)?,
                created_at: row.get(5)?,
            });
        }
        Ok(result)
    }

    pub fn delete(&mut self, id: i64) -> Result<(), Error> {
        self.conn.execute(
            "DELETE FROM staff WHERE staff_id = :id",
            named_params! { ":id": id },
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::db;

    #[test]
    fn test_insert_and_fetch() {
        let conn = db::open_in_memory().unwrap();
        let mut repo = StaffRepo::new(&conn);
        let staff = Staff::new("Jane".to_string(), "Smith".to_string());
        let id = repo.insert(&staff).unwrap();
        let fetched = repo.fetch_by_id(id).unwrap();
        assert_eq!(fetched.staff_first, "Jane");
        assert_eq!(fetched.staff_last, "Smith");
        assert_eq!(fetched.staff_email, "jane.smith@foxxything.com");
    }

    #[test]
    fn test_fetch_all() {
        let conn = db::open_in_memory().unwrap();
        let mut repo = StaffRepo::new(&conn);
        repo.insert(&Staff::new("Jane".to_string(), "Smith".to_string()))
            .unwrap();
        repo.insert(&Staff::new("John".to_string(), "Doe".to_string()))
            .unwrap();
        assert_eq!(repo.fetch_all().unwrap().len(), 2);
    }

    #[test]
    fn test_delete() {
        let conn = db::open_in_memory().unwrap();
        let mut repo = StaffRepo::new(&conn);
        let id = repo
            .insert(&Staff::new("Jane".to_string(), "Smith".to_string()))
            .unwrap();
        repo.delete(id).unwrap();
        assert!(repo.fetch_by_id(id).is_err());
    }
}
