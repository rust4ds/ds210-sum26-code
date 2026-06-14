use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};


//recursive helper function that iterates throughout all rows and return true or false 
pub fn row_matches (row: &Row, condition: &Condition, dataset: &Dataset) -> bool {
    match condition {
        //1. equal condition > all use this basically
        Condition::Equal(col_name, expected) => {
            //get index of column name
            let index = dataset.column_index (col_name);
            //looks at values of each row of that index and compare against expected
            row.get_value(index) == expected
        }
        //2. not condition
        Condition::Not(conditions) => {
            //not is just opposite of equal 
            !row_matches(row, conditions, dataset)
        }
        //3. and condition
        Condition::And(left, right) => {
            //unpacks box condition to left and right b/c it takes in two boxes
            row_matches(row, left, dataset) && row_matches(row, right, dataset)
        }
        //4. or condition
        Condition::Or(left, right)=> {
            //same as and 
            row_matches(row, left, dataset) || row_matches (row, right, dataset)
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    //create a new empty dataset first 
    let mut result = Dataset::new(dataset.columns().clone());
    //iterate through all rows in dataset, using recursive row matches function, if returns true, then add that row into new dataset
    for row in dataset.iter() { 
        if row_matches(row, filter, dataset) {
            result.add_row(row.clone());
        }
    }
    result
}
//split dataset into many datasets inside a HashMap that are mapped using the values of whatever column name
pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    //return copy of column name and type
    let columns = dataset.columns().clone();
    //gets column index with desired column name
    let column_index = dataset.column_index(group_by_column);
    let mut groups: HashMap<Value, Dataset> = HashMap::new();

    //moves the row out of old dataset into new Hashmap
    for row in dataset.into_iter() {
        //take value at column index in each row
        let key = row.get_value(column_index).clone();  
        //does it all in one: if already has a key then go to it, if not create a new empty dataset and add rows
        groups.entry(key).or_insert_with(||Dataset::new(columns.clone())).add_row(row);
    }
    groups
}

//create a helper function to sum up values in column 
fn sum_column(dataset: &Dataset, col_name: &String) -> i32 {
    //get index of column name
    let col_index = dataset.column_index(col_name);
    let mut sum = 0;
    //first check that value of column is integer, then index the row value and add to sum
    for row in dataset.iter() {
        if let Value::Integer(val) = row.get_value(col_index) {
            sum += val;
        }
    }
    sum
   
}

//performs math on the data, returns hashmap that maps whatever column into count, sum or avg 
pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    //moves the values out of old hashmap into a new one
    dataset.into_iter().map(|(key, group)| {
        let agg_value = match aggregation {
            Aggregation::Count(_) => {
                Value::Integer(group.len() as i32) //just get len for count
            }
            Aggregation::Sum(col_name) => {
                Value::Integer(sum_column(&group, col_name)) //use helper function
            }
            Aggregation::Average(col_name)=> {
                let total = sum_column(&group, col_name);
                Value::Integer(total / group.len() as i32)
            }
        };
        (key, agg_value)
    })
    .collect()
}


pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}