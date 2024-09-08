use reqwest::Error;
use retrieval_context::get_sec_user_client;
use state_maschine::prelude::*;
use std::fmt;

pub mod retrieval_context;
pub mod retrieval_data;

pub use retrieval_context::RetrievalContext;
pub use retrieval_data::RetrievalData;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Retrieval {
    input: RetrievalData,
    output: Option<RetrievalData>,
    context: RetrievalContext,
}

impl State for Retrieval {
    type InputData = RetrievalData;
    type OutputData = RetrievalData;
    type Context = RetrievalContext;

    fn get_state_name(&self) -> impl ToString {
        "retrieval"
    }

    fn get_input_data(&self) -> &RetrievalData {
        &self.input
    }

    fn compute_output_data(&mut self) {
        self.output = Some(RetrievalData::default());
    }

    fn get_output_data(&self) -> Option<&RetrievalData> {
        self.output.as_ref()
    }

    fn get_context_data(&self) -> &RetrievalContext {
        &self.context
    }
}

impl Retrieval {
    /// Computes the output by retrieving data from the SEC API.
    ///
    /// This function sends an HTTP GET request to the SEC's API using the CIK (Central Index Key)
    /// to retrieve company facts in JSON format. The result is printed out for the first 100
    /// characters.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - If the HTTP client cannot be built (see [`get_sec_user_client`] for details).
    /// - If the request to the SEC API fails (e.g., network errors, invalid response).
    /// - If the body of the HTTP response cannot be retrieved or parsed.
    ///
    /// The errors are wrapped in a [`reqwest::Error`] or any custom `Error` type if applicable.
    pub async fn compute_output_new(&self) -> Result<(), Error> {
        let cik = self.get_context_data().cik();
        let url = format!("https://data.sec.gov/api/xbrl/companyfacts/CIK{cik}.json");

        let client = get_sec_user_client()?;

        let body = client.get(&url).send().await?.text().await?;

        println!(
            "Did the retrieval process for this cik: {cik} with this body: {}...",
            &body[..100]
        );

        Ok(())
    }
}
impl fmt::Display for Retrieval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Retrieval State Summary\n\
             ———————————————————————\n\
             Context:\n{}\n\
             Input Data:\n{}\n\
             Output Data:\n\t{}",
            self.context,
            self.input,
            self.output.as_ref().map_or_else(
                || "None".to_string(),
                |output_data| format!("{output_data}")
            )
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{fmt::Debug, hash::Hash};

    #[test]
    fn should_return_name_of_retrieval_state_when_in_retrieval_state() {
        let retrieval_state = Retrieval::default();

        let expected_result = String::from("retrieval");

        let result = retrieval_state.get_state_name().to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_retrieval_data_struct_as_input_data_when_output_data_has_not_been_computed_in_state(
    ) {
        let retrieval_state = Retrieval::default();

        let expected_result = &RetrievalData::default();

        let result = retrieval_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_retrieval_data_struct_as_input_data_when_in_initial_retrieval_state() {
        let retrieval_state = Retrieval::default();

        let expected_result = &RetrievalData::default();

        let result = retrieval_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic(expected = "output should not be empty")]
    fn should_panic_when_trying_to_access_output_data_before_it_has_been_computed_in_state() {
        let retrieval_state = Retrieval::default();

        let _result = retrieval_state
            .get_output_data()
            .expect("The output should not be empty.");
    }

    #[test]
    fn should_return_false_when_state_has_not_computed_the_output() {
        let retrieval_state = Retrieval::default();

        let expected_result = false;

        let result = retrieval_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_true_when_state_has_computed_the_output() {
        let mut retrieval_state = Retrieval::default();

        let expected_result = true;

        retrieval_state.compute_output_data();
        let result = retrieval_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_context_data_when_in_initial_state() {
        let retrieval_state = Retrieval::default();

        let expected_result = &RetrievalContext::default();

        let result = retrieval_state.get_context_data();

        assert_eq!(result, expected_result);
    }

    fn implements_auto_traits<T: Sized + Send + Sync + Unpin>() {}
    #[test]
    fn should_still_implement_auto_traits_traits_when_implementing_state_trait() {
        implements_auto_traits::<Retrieval>();
    }

    fn implements_send<T: Send>() {}
    fn implements_sync<T: Sync>() {}

    #[test]
    fn should_implement_send_when_implementing_state_trait() {
        implements_send::<Retrieval>();
    }

    #[test]
    fn should_implement_sync_when_implementing_state_trait() {
        implements_sync::<Retrieval>();
    }

    #[test]
    fn should_be_thread_safe_when_implementing_state_trait() {
        implements_send::<Retrieval>();
        implements_sync::<Retrieval>();
    }

    fn implements_sized<T: Sized>() {}
    #[test]
    fn should_be_sized_when_implementing_state_trait() {
        implements_sized::<Retrieval>();
    }

    fn implements_hash<T: Hash>() {}
    #[test]
    fn should_implement_hash_when_implementing_state_trait() {
        implements_hash::<Retrieval>();
    }

    fn implements_partial_eq<T: PartialEq>() {}
    #[test]
    fn should_implement_partial_eq_when_implementing_state_trait() {
        implements_partial_eq::<Retrieval>();
    }

    fn implements_eq<T: Eq>() {}
    #[test]
    fn should_implement_eq_when_implementing_state_trait() {
        implements_eq::<Retrieval>();
    }

    fn implements_partial_ord<T: PartialOrd>() {}
    #[test]
    fn should_implement_partial_ord_when_implementing_state_trait() {
        implements_partial_ord::<Retrieval>();
    }

    fn implements_ord<T: Ord>() {}
    #[test]
    fn should_implement_ord_when_implementing_state_trait() {
        implements_ord::<Retrieval>();
    }

    fn implements_default<T: Default>() {}
    #[test]
    fn should_implement_default_when_implementing_state_trait() {
        implements_default::<Retrieval>()
    }

    fn implements_debug<T: Debug>() {}
    #[test]
    fn should_implement_debug_when_implementing_state_trait() {
        implements_debug::<Retrieval>();
    }

    fn implements_clone<T: Clone>() {}
    #[test]
    fn should_implement_clone_when_implementing_state_trait() {
        implements_clone::<Retrieval>();
    }

    fn implements_unpin<T: Unpin>() {}
    #[test]
    fn should_implement_unpin_when_implementing_state_trait() {
        implements_unpin::<Retrieval>();
    }

    #[test]
    fn should_return_default_context_data_when_called_with_state_reference() {
        let retrieval_state = &Retrieval::default();
        let ref_to_retrieval_state = &Retrieval::default();

        let expected_result = retrieval_state.get_context_data();

        let result = ref_to_retrieval_state.get_context_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_true_when_reference_state_has_computed_the_output() {
        let ref_to_retrieval_state = &mut Retrieval::default();

        let expected_result = true;

        ref_to_retrieval_state.compute_output_data();
        let result = ref_to_retrieval_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_false_when_reference_state_has_not_computed_the_output() {
        let ref_to_retrieval_state = &mut Retrieval::default();

        let expected_result = false;

        let result = ref_to_retrieval_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic(expected = "output should not be empty")]
    fn should_panic_when_trying_to_access_output_data_before_it_has_been_computed_in_reference_state(
    ) {
        let ref_to_retrieval_state = &Retrieval::default();

        let _result = ref_to_retrieval_state
            .get_output_data()
            .expect("The output should not be empty.");
    }

    #[test]
    fn should_return_name_of_retrieval_state_when_calling_reference_to_retrieval_state() {
        let ref_to_retrieval_state = &Retrieval::default();

        let expected_result = String::from("retrieval");

        let result = ref_to_retrieval_state.get_state_name().to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_state_data_as_input_data_when_output_data_has_not_been_computed_in_reference_state(
    ) {
        let ref_to_retrieval_state = &Retrieval::default();

        let expected_result = &RetrievalData::default();

        let result = ref_to_retrieval_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_state_data_as_input_data_when_reference_retrieval_state_in_initial_state(
    ) {
        let ref_to_retrieval_state = &Retrieval::default();

        let expected_result = &RetrievalData::default();

        let result = ref_to_retrieval_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_not_change_input_data_when_computing_output_data() {
        let mut retrieval_state = Retrieval::default();

        let expected_result = &retrieval_state.get_input_data().clone();

        retrieval_state.compute_output_data();
        let result = retrieval_state.get_input_data();

        assert_eq!(result, expected_result)
    }
}