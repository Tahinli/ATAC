use crate::app::app::App;
use crate::app::app_states::AppState;
use crate::request::auth::Auth;
use crate::request::request::{Request};

impl<'a> App<'a> {
    pub fn reset_inputs(&mut self) {
        self.url_text_input.reset_input();
        self.auth_basic_username_text_input.reset_input();
        self.auth_basic_password_text_input.reset_input();
    }

    pub fn update_inputs(&mut self) {
        self.reset_inputs();

        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        self.url_text_input.enter_str(selected_request.url);

        match &selected_request.auth {
            Auth::NoAuth => {}
            Auth::BasicAuth(username, password) => {
                self.auth_basic_username_text_input.enter_str(username);
                self.auth_basic_password_text_input.enter_str(password);
            }
            Auth::BearerToken(_token) => {}
        }

        let body = selected_request.body.get_body_as_string();
        self.refresh_body_textarea(body);
    }

    pub fn select_request(&mut self) {
        self.collection.select();
        self.result_scrollbar.set_scroll(0);

        if self.collection.selected.is_some() {
            self.update_inputs();
            self.state = AppState::SelectedRequest;
        }
    }

    pub fn unselect_request(&mut self) {
        self.reset_inputs();
        self.collection.unselect();
    }

    pub fn new_request(&mut self) {
        let new_request_name = &self.new_request_input.text;

        if new_request_name.len() == 0 {
            return;
        }

        let new_request = Request::<'a> {
            name: new_request_name.clone().leak(),
            ..Default::default()
        };

        self.collection.items.push(new_request);

        self.state = AppState::Normal;
    }

    pub fn delete_request(&mut self) {
        if let Some(selected_request_index) = self.collection.state.selected() {
            self.collection.unselect();
            self.collection.items.remove(selected_request_index);
        }
    }
}