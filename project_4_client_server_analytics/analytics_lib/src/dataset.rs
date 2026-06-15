use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
//ColumnType stores string and integer
pub enum ColumnType {
    String,
    Integer,
}

//Value stores string or integer 
#[derive(Clone, PartialEq, Hash, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Integer(i32),
}
//convert value into string
impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::String(value) => value.to_string(),
            Value::Integer(value) => value.to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Row {
    values: Vec<Value>,
}
impl Row {
    pub fn new(values: Vec<Value>) -> Row {
        return Row { values };
    }
    pub fn get_values(&self) -> &Vec<Value> { //returns one row 
        return &self.values;
    }
    pub fn get_value(&self, index: usize) -> &Value { //returns one value (either string or integer)
        return &self.values[index];
    }
    pub fn move_values(self) -> Vec<Value> { //moves values into new vector 
        return self.values;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Dataset {
    columns: Vec<(String, ColumnType)>,
    rows: Vec<Row>,
}
impl Dataset {
    pub fn new(columns: Vec<(String, ColumnType)>) -> Dataset { //creates a new dataset
        return Dataset {
            columns,
            rows: Vec::new(),
        };
    }
    pub fn add_row(&mut self, row: Row) { //adds a new row
        self.rows.push(row);
    }

    pub fn columns(&self) -> &Vec<(String, ColumnType)> { //returns names of columns
        return &self.columns;
    }
    pub fn column_type(&self, column_name: &String) -> &ColumnType { //returns if its string or integer
       let i = self.column_index(column_name);
        return &self.columns[i].1;
    }
    pub fn column_index(&self, column_name: &String) -> usize { //finds column needed and returns index 
        for i in 0..self.columns.len() {
            let (cname, _ctype) = &self.columns[i];
            if cname == column_name {
                return i;
            }
        }
        panic!("Column {} not found", column_name);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Row> { // gives references to each row of dataset 
        return self.rows.iter();
    }

    pub fn into_iter(self) -> std::vec::IntoIter<Row> { //gives you the actual rows of each dataset (moving it out)
        return self.rows.into_iter();
    }

    pub fn len(&self) -> usize { //finds how many rows are in dataset
        return self.rows.len();
    }
}

impl Debug for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dataset:")?;
        write!(f, "|")?;
        for (colname, coltype) in &self.columns {
            let description = format!("{colname}: {coltype:?}");
            write!(f, " {description: <28}|")?;
        }
        writeln!(f, "")?;

        write!(f, "|")?;
        for _ in &self.columns {
            write!(f, "=============================|")?;
        }
        writeln!(f, "")?;

        for row in &self.rows {
            write!(f, "|")?;
            for value in row.get_values() {
                write!(f, " {: <28}|", value.to_string())?;
            }
            writeln!(f, "")?;
        }
        return Ok(());
    }
}
impl Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return Debug::fmt(self, f);
    }
}

impl PartialEq for Dataset {
    fn eq(&self, other: &Self) -> bool {
        if self.columns != other.columns {
            return false;
        }

        let mut rows = self.rows.clone();
        rows.sort();
        let mut rows2 = other.rows.clone();
        rows2.sort();

        return rows == rows2;
    }
}