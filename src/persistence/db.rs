use rusqlite::{Connection, Error};

pub fn open(path: &str) -> Result<Connection, Error> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    create_tables(&conn)?;
    Ok(conn)
}

pub fn open_in_memory() -> Result<Connection, Error> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    create_tables(&conn)?;
    Ok(conn)
}

fn create_tables(conn: &Connection) -> Result<(), Error> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS staff (
            staff_id    INTEGER PRIMARY KEY AUTOINCREMENT,
            staff_first TEXT                           NOT NULL,
            staff_last  TEXT                           NOT NULL,
            staff_email TEXT                           NOT NULL UNIQUE,
            staff_title TEXT                           NOT NULL,
            created_at  TEXT DEFAULT (datetime('now')) NOT NULL
        );

        CREATE TABLE IF NOT EXISTS courses (
            course_id   INTEGER PRIMARY KEY AUTOINCREMENT,
            course_name TEXT                           NOT NULL UNIQUE,
            staff_id    INTEGER                        NOT NULL REFERENCES staff,
            created_at  TEXT DEFAULT (datetime('now')) NOT NULL
        );

        CREATE TABLE IF NOT EXISTS assignments (
            assignment_id   INTEGER PRIMARY KEY AUTOINCREMENT,
            course_id       INTEGER                        NOT NULL REFERENCES courses,
            assignment_name TEXT                           NOT NULL,
            weight          REAL    DEFAULT 1.0            NOT NULL,
            created_at      TEXT    DEFAULT (datetime('now')) NOT NULL
        );

        CREATE TABLE IF NOT EXISTS student (
            student_id    INTEGER PRIMARY KEY AUTOINCREMENT,
            student_first TEXT                           NOT NULL,
            student_last  TEXT                           NOT NULL,
            student_email TEXT                           NOT NULL UNIQUE,
            created_at    TEXT DEFAULT (datetime('now')) NOT NULL
        );

        CREATE TABLE IF NOT EXISTS enrollments (
            enrollment_id INTEGER PRIMARY KEY AUTOINCREMENT,
            student_id    INTEGER                        NOT NULL REFERENCES student,
            course_id     INTEGER                        NOT NULL REFERENCES courses,
            enrolled_at   TEXT DEFAULT (datetime('now')) NOT NULL,
            CONSTRAINT enrollments_uk UNIQUE (student_id, course_id)
        );

        CREATE TABLE IF NOT EXISTS assignment_grades (
            enrollment_id INTEGER                        NOT NULL REFERENCES enrollments,
            assignment_id INTEGER                        NOT NULL REFERENCES assignments,
            grade         REAL    DEFAULT 0              NOT NULL,
            graded_at     TEXT    DEFAULT (datetime('now')) NOT NULL,
            CONSTRAINT assignment_grades_pk PRIMARY KEY (enrollment_id, assignment_id)
        );

        CREATE VIEW IF NOT EXISTS course_assignment_averages AS
        SELECT c.course_id,
               c.course_name,
               a.assignment_id,
               a.assignment_name,
               a.weight,
               ROUND(AVG(ag.grade), 2) AS class_average,
               COUNT(ag.grade)         AS students_graded
        FROM assignments a
                 JOIN courses c ON c.course_id = a.course_id
                 LEFT JOIN assignment_grades ag ON ag.assignment_id = a.assignment_id
        GROUP BY a.assignment_id;

        CREATE VIEW IF NOT EXISTS student_final_grades AS
        SELECT s.student_id,
               s.student_first || ' ' || s.student_last AS student_name,
               c.course_id,
               c.course_name,
               st.staff_first || ' ' || st.staff_last   AS teacher_name,
               ROUND(SUM(ag.grade * a.weight) / SUM(a.weight), 2) AS final_grade,
               COUNT(ag.assignment_id)                   AS assignments_graded
        FROM enrollments e
                 JOIN student s ON s.student_id = e.student_id
                 JOIN courses c ON c.course_id = e.course_id
                 JOIN staff st ON st.staff_id = c.staff_id
                 JOIN assignment_grades ag ON ag.enrollment_id = e.enrollment_id
                 JOIN assignments a ON a.assignment_id = ag.assignment_id
        GROUP BY e.enrollment_id;

        CREATE VIEW IF NOT EXISTS ungraded_students AS
        SELECT s.student_id,
               s.student_first || ' ' || s.student_last AS student_name,
               c.course_name,
               a.assignment_name
        FROM enrollments e
                 JOIN student s ON s.student_id = e.student_id
                 JOIN courses c ON c.course_id = e.course_id
                 JOIN assignments a ON a.course_id = e.course_id
                 LEFT JOIN assignment_grades ag
                           ON ag.enrollment_id = e.enrollment_id
                           AND ag.assignment_id = a.assignment_id
        WHERE ag.grade IS NULL;
    ")?;

    Ok(())
}