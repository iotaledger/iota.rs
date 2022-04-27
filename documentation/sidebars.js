/**
 * * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

module.exports = {
    docs: [{
        type: 'doc',
        id: 'welcome',
    },
        {
            type: 'doc',
            id: 'overview',
        },
        {
            type: 'doc',
            id: 'getting_started',
        },
        {
            type: 'category',
            label: 'Key Concepts',
            items:[
                    'key_concepts/seeds',
                    'key_concepts/address_key_space',
                    'key_concepts/messages_payloads_and_transactions'
            ]
        },
        {
            type: 'category',
            label: 'Examples',
            items:[
                'how_tos/get_info',
                'how_tos/generate_seed',
                'how_tos/generate_addresses',
                'how_tos/get_balance',
                'how_tos/get_outputs',
                'how_tos/simple_message',
                'how_tos/get_message_data',
                'how_tos/data_message'
            ]
        },
        {
            type: 'category',
            label: 'Libraries',
            collapsed: false,
            items: [{
                type: 'doc',
                id: 'libraries/overview',
                label: 'Overview',
            },
                {
                    type: 'category',
                    label: 'Rust',
                    items: [
                        {
                            type: 'doc',
                            id: 'libraries/rust/getting_started',
                            label: 'Getting Started',
                        },
                        {
                            type: 'doc',
                            id: 'libraries/rust/examples',
                            label: 'Examples'
                        },
                        {
                            type: 'doc',
                            id: 'libraries/rust/api_reference',
                            label: 'API Reference'
                        },
                    ]
                },
                {
                    type: 'category',
                    label: 'Node.js',
                    items: [
                        {
                            type: 'doc',
                            id: 'libraries/nodejs/getting_started',
                            label: 'Getting Started'
                        },
                        {
                            type: 'category',
                            label: 'Examples',
                            items: [
                                'libraries/nodejs/examples/transaction',
                                'libraries/nodejs/examples/mqtt',
                            ],
                        },
                        {
                            type: 'doc',
                            id: 'libraries/nodejs/api_reference',
                            label: 'API Reference'
                        },
                    ]
                },
                {
                    type: 'category',
                    label: 'Python',
                    items: [
                        {
                            type: 'doc',
                            id: 'libraries/python/getting_started',
                            label: 'Getting Started'
                        },
                        {
                            type: 'category',
                            label: 'Examples',
                            items: [
                                'libraries/python/examples/transaction',
                            ]
                        },
                        {
                            type: 'doc',
                            id: 'libraries/python/api_reference',
                            label: 'API Reference'
                        },
                    ]
                },
                {
                    type: 'category',
                    label: 'Java',
                    items: [
                        {
                            type: 'doc',
                            id: 'libraries/java/getting_started',
                            label: 'Getting Started'
                        },
                        {
                            type: 'doc',
                            id: 'libraries/java/examples',
                            label: 'Examples'
                        },
                        {
                            type: 'doc',
                            id: 'libraries/java/api_reference',
                            label: 'API Reference'
                        },
                    ]
                },
                {
                    type: 'category',
                    label: 'Wasm',
                    items: [
                        {
                            type: 'doc',
                            id: 'libraries/wasm/getting_started',
                            label: 'Getting Started'
                        },
                        {
                            type: 'category',
                            label: 'Examples',
                            items: [
                                'libraries/wasm/examples/transaction',
                            ],
                        },
                        {
                            type: 'doc',
                            id: 'libraries/wasm/api_reference',
                            label: 'API Reference'
                        },
                    ]
                }
            ]
        },
        {
            type: 'doc',
            id: 'specs',
            label: 'Specification',
        },
        {
            type: 'doc',
            id: 'troubleshooting',
            label: 'Troubleshooting'
        },
        {
            type: 'doc',
            id: 'contribute',
            label: 'Contribute',
        }
    ]
};
