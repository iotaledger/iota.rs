// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::Client;
use pyo3::{conversion::ToPyObject, exceptions, prelude::*, types::PyTuple};
use std::convert::TryInto;

#[pymethods]
impl Client {
    fn subscribe_topic(&mut self, topic: &str, callback: PyObject) -> PyResult<()> {
        self.subscribe_topics([topic].to_vec(), callback)
    }

    fn subscribe_topics(&mut self, topics: Vec<&str>, callback: PyObject) -> PyResult<()> {
        let result = crate::block_on(
            self.client
                .subscriber()
                .with_topics(
                    topics
                        .iter()
                        .map(|&topic| topic.to_owned()[..].try_into().unwrap())
                        .collect(),
                )
                .subscribe(move |event| {
                    // We need to clone it because `callback` has type `Py<PyAny>`,
                    // which does not implement the `Copy` trait
                    let callback_copy = callback.clone();
                    // Pare the event_string first to reduce the gil blocking period
                    let event_string = serde_json::to_string(&event).unwrap();
                    crate::spawn_blocking(move || {
                        let gil = Python::acquire_gil();
                        let py = gil.python();
                        let args = PyTuple::new(py, &[event_string]);
                        callback_copy.call1(py, args).unwrap_or_else(|_| {
                            PyErr::new::<exceptions::PyTypeError, _>("Unable to use the python callback function!")
                                .to_object(py)
                        })
                    });
                }),
        );
        match result {
            Err(err) => Err(PyErr::new::<exceptions::PyTypeError, _>(err.to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn unsubscribe(&mut self) -> PyResult<()> {
        let result = crate::block_on(self.client.subscriber().unsubscribe());
        match result {
            Err(err) => Err(PyErr::new::<exceptions::PyTypeError, _>(err.to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn unsubscribe_topics(&mut self, topics: Vec<&str>) -> PyResult<()> {
        let result = crate::block_on(
            self.client
                .subscriber()
                .with_topics(
                    topics
                        .iter()
                        .map(|&topic| topic.to_owned()[..].try_into().unwrap())
                        .collect(),
                )
                .unsubscribe(),
        );
        match result {
            Err(err) => Err(PyErr::new::<exceptions::PyTypeError, _>(err.to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn disconnect(&mut self) -> PyResult<()> {
        let result = crate::block_on(self.client.subscriber().disconnect());
        match result {
            Err(err) => Err(PyErr::new::<exceptions::PyTypeError, _>(err.to_string())),
            Ok(()) => Ok(()),
        }
    }
}
