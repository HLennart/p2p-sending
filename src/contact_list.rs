use iced::{button, text_input, Button, Column, Element, Row, Text, TextInput};

#[derive(Debug, Clone)]
pub enum ContactMessage {
    EditContact,
    DoneEditing,
    ContactChanged(usize, Change),
    NameChanged(String),
    IpChanged(String),
}

#[derive(Debug, Clone)]
pub enum Change {
    EditContact,
    DoneEditing,
    NameChanged(String),
    IpChanged(String),
}

enum ContactObject {
    ExistingContact(Contact),
    CurrentlyEditing(EditContact),
}

pub struct ContactList {
    contacts: Vec<ContactObject>,
}

impl ContactList {
    pub fn view(&mut self) -> Element<ContactMessage> {
        self.contacts
            .iter_mut()
            .enumerate()
            .fold(Column::new(), |c, (index, contact)| {
                c.push(
                    contact
                        .view()
                        .map(move |msg| ContactMessage::ContactChanged(index, msg)),
                )
            })
            .into()
    }

    pub fn update(&mut self, msg: ContactMessage) {
        match msg {
            ContactMessage::EditContact => {}
            ContactMessage::DoneEditing => {}
            ContactMessage::ContactChanged(index, change) => {
                if let Some(contact) = self.contacts.get_mut(index) {
                    contact.update(change);
                }
            }
            ContactMessage::NameChanged(_) => {}
            ContactMessage::IpChanged(_) => {}
        }
    }
}

impl Default for ContactList {
    fn default() -> Self {
        use ContactObject::ExistingContact;
        Self {
            contacts: vec![
                ExistingContact(Contact::new(0, "Tom", "1.1.1.1")),
                ExistingContact(Contact::new(1, "Grete", "2.2.2.2")),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Contact {
    index: usize,
    name: String,
    ip: String,
    edit_button: button::State,
}

impl Contact {
    pub fn new<S: Into<String>>(index: usize, name: S, ip: S) -> Self {
        Self {
            index,
            name: name.into(),
            ip: ip.into(),
            edit_button: button::State::new(),
        }
    }

    fn view(&mut self) -> Element<Change> {
        Row::new()
            .push(
                Column::new()
                    .push(Text::new(self.name.clone()))
                    .push(Text::new(self.ip.clone())),
            )
            .push(
                Button::new(&mut self.edit_button, Text::new("edit")).on_press(Change::EditContact),
            )
            .into()
    }
}

impl ContactObject {
    fn view(&mut self) -> Element<Change> {
        match self {
            ContactObject::ExistingContact(contact) => contact.view(),
            ContactObject::CurrentlyEditing(editing_contact) => editing_contact.view(),
        }
    }

    fn update(&mut self, msg: Change) {
        match msg {
            Change::EditContact => {
                if let ContactObject::ExistingContact(contact) = self {
                    *self = ContactObject::CurrentlyEditing(EditContact::new(
                        contact.index,
                        contact.name.clone(),
                        contact.ip.clone(),
                    ));
                }
            }
            Change::DoneEditing => {
                if let ContactObject::CurrentlyEditing(contact) = self {
                    *self = ContactObject::ExistingContact(Contact::new(
                        contact.index,
                        contact.name.clone(),
                        contact.ip.clone(),
                    ));
                }
            }
            Change::NameChanged(name) => {
                if let ContactObject::CurrentlyEditing(contact) = self {
                    contact.name = name;
                }
            }
            Change::IpChanged(ip) => {
                if let ContactObject::CurrentlyEditing(contact) = self {
                    contact.ip = ip;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct EditContact {
    index: usize,
    previous_name: String,
    name: String,
    name_input: text_input::State,
    previous_ip: String,
    ip: String,
    ip_input: text_input::State,
    save_button: button::State,
}

impl EditContact {
    pub fn new<S: Into<String> + Clone>(index: usize, name: S, ip: S) -> Self {
        Self {
            index,
            previous_name: name.clone().into(),
            name: name.into(),
            name_input: text_input::State::focused(),
            previous_ip: ip.clone().into(),
            ip: ip.into(),
            ip_input: text_input::State::default(),
            save_button: button::State::default(),
        }
    }

    fn view(&mut self) -> Element<Change> {
        let name_text_field = TextInput::new(
            &mut self.name_input,
            &self.previous_name,
            &self.name.clone(),
            Change::NameChanged,
        ).max_width(16).on_submit(Change::DoneEditing);

        let ip_text_field = TextInput::new(
            &mut self.ip_input,
            &self.previous_name,
            &self.ip.clone(),
            Change::IpChanged,
        ).max_width(16).on_submit(Change::DoneEditing);

        Row::new()
            .push(Column::new().push(name_text_field).push(ip_text_field))
            .push(
                Button::new(&mut self.save_button, Text::new("save")).on_press(Change::DoneEditing),
            ) // INDEX??
            .into()
    }
}
