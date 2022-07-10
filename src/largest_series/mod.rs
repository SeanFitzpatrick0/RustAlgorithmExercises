/// Finds continuous sub series of a set length with the largest sum
///
/// # Arguments
/// * `series` - The entire series to search
/// * `span` - The size of the sub series to find
///
/// # Example
/// ```
/// use rust_exercises::largest_series::get_largest_series;
///
/// // Given
/// let values = [1, 2, 3, 4, 5, 4, 2, 1];
///
/// // When
/// let result = get_largest_series(&values, 3);
///
/// // Then
/// let expected_largest_series = &values[3..5];
/// assert_eq!(result, expected_largest_series);
/// ```
pub fn get_largest_series(series: &[u32], span: usize) -> &[u32] {
    if series.len() < span {
        // Check that there is enough values in series to execute the query
        panic!(
            "Span is larger than series. span = {span} series length {}",
            series.len()
        );
    }

    let mut largest_series = (0 as usize, span.clone() - 1);
    let mut current_series = largest_series.clone();

    let mut largest_series_sum: u64 = series[..current_series.1 + 1]
        .iter()
        .map(|&x| x as u64)
        .sum();
    let mut current_series_sum = largest_series_sum.clone();

    while current_series.1 < series.len() - 1 {
        // Slide series
        let prev_start_value = series[current_series.0];
        current_series.0 += 1;
        current_series.1 += 1;
        let new_end_value = series[current_series.1];

        // Check for new largest
        current_series_sum += new_end_value as u64;
        current_series_sum -= prev_start_value as u64;
        if current_series_sum > largest_series_sum {
            largest_series_sum = current_series_sum;
            largest_series = current_series.clone();
        }
    }

    return &series[largest_series.0..largest_series.1];
}
