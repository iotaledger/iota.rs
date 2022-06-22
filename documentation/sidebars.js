/**
 * * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */
module.exports = {
  docs: [
    {
      type: "doc",
      id: "welcome",
    },
    {
      type: "doc",
      id: "overview",
    },
    {
      type: "category",
      label: 'Getting Started',
      items:[
        {
          type: "doc",
          id: "getting_started/required_prior_knowledge",
        },
        {
          type: "category",
          label: "Java",
          items:[
            'getting_started/java/getting_started',
            'getting_started/java/android_development',
          ]
        },
        {
          type: "doc",
          id: "getting_started/nodejs",
          label: 'Nodejs'
        },
        {
          type: "doc",
          id: "getting_started/python",
          label: 'Python'
        },
        {
          type: "doc",
          id: "getting_started/rust",
          label: 'Rust'
        },
        {
          type: "doc",
          id: "getting_started/wasm",
          label: 'Wasm'
        },
      ]
    },
    {
      type: "category",
      label: "Explanations",
      items: [
        "explanations/seeds",
        "explanations/address_key_space",
        "explanations/messages_payloads_and_transactions",
      ],
    },
    {
      type: "category",
      label: "Examples",
      items: [
        "examples/running_examples",
        "examples/get_info",
        "examples/generate_seed",
        "examples/generate_addresses",
        "examples/get_balance",
        "examples/get_outputs",
        "examples/simple_message",
        "examples/get_message_data",
        "examples/data_message",
        "examples/transaction",
        "examples/mqtt",
      ],
    },
    {
      type: "category",
      label: "API Reference",
      items: [
        {
          type: "doc",
          id: "libraries/java/api_reference",
          label: "Java",
        },
        {
          type: "doc",
          id: "libraries/nodejs/api_reference",
          label: "Node.js",
        },
        {
          type: "doc",
          id: "libraries/python/api_reference",
          label: "Python",
        },
        {
          type: "doc",
          id: "libraries/rust/api_reference",
          label: "Rust",
        },
        {
          type: "doc",
          id: "libraries/wasm/api_reference",
          label: "Wasm",
        },
      ],
    },
    {
      type: "doc",
      id: "specs",
      label: "Specification",
    },
    {
      type: "doc",
      id: "troubleshooting",
      label: "Troubleshooting",
    },
    {
      type: "doc",
      id: "contribute",
      label: "Contribute",
    },
  ],
};
