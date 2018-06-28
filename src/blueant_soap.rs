extern crate pyo3;

use pyo3::{ObjectProtocol, PyDict, PyResult, Python};

pub struct BaseService {
    python: Python;
    client: &PyObjectRef;
}

impl BaseService {

    pub fn new(): self {
        let gil = Python::acquire_gil();
        let python = gil.python();

        let zeep_module = self.python.import("zeep").unwrap();
        let locals = PyDict::new(self.python);
        locals.set_item("zeep", zeep_module).unwrap();

        let client = python.eval(
            "zeep.Client('https://blueant.sinnerschrader.com/blueant/services/BaseService?wsdl')",
            None,
            Some(&locals),
        ).unwrap();

        BaseService {
            python,
            client
        }
    }

    pub fn login(&self, username: &String, password: &String) {
        let locals = PyDict::new(self.python);
        locals.set_item("client", self.client).unwrap();
        locals.set_item("username", username).unwrap();
        locals.set_item("password", password).unwrap();

        println!(
            "{:?}",
            self.python.eval(
                "client.service.Login(username, password)",
                None,
                Some(&locals)
            ).unwrap()
                .get("sessionID")
                .unwrap()
        );
    }
}

 
    