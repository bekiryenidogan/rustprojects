//Define a struct called FilterCondition with a single field of the desired type for filtering.
struct FilterCondition<T> {
    value: T,
 }
 
 impl<T: PartialEq> FilterCondition<T> {
    /*  Implement a method called is_match on the FilterCondition struct that takes a reference to an item
    of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.*/
    fn is_match(&self, item: &T) -> bool {
        item == &self.value
    }
 }
 
 /*  Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition
  object as arguments. The function should iterate over the elements in the collection and return a new collection containing 
  only the elements that match the filter condition.*/
 fn custom_filter<T, F>(collection: Vec<T>, condition: F) -> Vec<T>
 where
    F: Fn(&T) -> bool,
    T: Clone,
 {
    collection
        .into_iter()
        .filter(|item| condition(item))
        .collect()
 }
 
 fn main() {
    // In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.
    let original_collection = vec![10, 20, 30, 40, 50, 60];
    let filter_condition = FilterCondition { value: 20 };
 
    // Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.
    let filtered_collection = custom_filter(original_collection.clone(), |item| {
        filter_condition.is_match(item)
    });
 
    // Print the filtered result to the console.
    println!("Orijinal koleksiyon: {:?}", original_collection);
    println!("Filtrelenmiş koleksiyon: {:?}", filtered_collection);
 }
 