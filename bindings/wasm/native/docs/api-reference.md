## Classes

<dl>
<dt><a href="#Client">Client</a></dt>
<dd></dd>
<dt><a href="#ClientBuilder">ClientBuilder</a></dt>
<dd></dd>
<dt><a href="#MessageBuilder">MessageBuilder</a></dt>
<dd></dd>
<dt><a href="#MessageGetter">MessageGetter</a></dt>
<dd></dd>
</dl>

## Functions

<dl>
<dt><a href="#start">start()</a></dt>
<dd><p>Initializes the console error panic hook for better error messages</p>
</dd>
</dl>

<a name="Client"></a>

## Client
**Kind**: global class  

* [Client](#Client)
    * [.getInfo()](#Client+getInfo) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.message()](#Client+message) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
    * [.getMessage()](#Client+getMessage) ⇒ [<code>MessageGetter</code>](#MessageGetter)

<a name="Client+getInfo"></a>

### client.getInfo() ⇒ <code>Promise.&lt;any&gt;</code>
Get the nodeinfo.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+message"></a>

### client.message() ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
Send a message to the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getMessage"></a>

### client.getMessage() ⇒ [<code>MessageGetter</code>](#MessageGetter)
Get a message from the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="ClientBuilder"></a>

## ClientBuilder
**Kind**: global class  

* [ClientBuilder](#ClientBuilder)
    * [.node(url)](#ClientBuilder+node) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
    * [.primaryNode(url, jwt, username, password)](#ClientBuilder+primaryNode) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
    * [.primaryPowNode(url, jwt, username, password)](#ClientBuilder+primaryPowNode) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
    * [.permanode(url, jwt, username, password)](#ClientBuilder+permanode) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
    * [.nodeAuth(url, jwt, username, password)](#ClientBuilder+nodeAuth) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
    * [.nodeSyncInterval(value)](#ClientBuilder+nodeSyncInterval) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
    * [.nodeSyncDisabled()](#ClientBuilder+nodeSyncDisabled) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
    * [.quorum(value)](#ClientBuilder+quorum)
    * [.quorumSize(value)](#ClientBuilder+quorumSize)
    * [.quorumThreshold(value)](#ClientBuilder+quorumThreshold)
    * [.tipsInterval(value)](#ClientBuilder+tipsInterval)
    * [.requestTimeout(value)](#ClientBuilder+requestTimeout)
    * [.build()](#ClientBuilder+build) ⇒ [<code>Client</code>](#Client)

<a name="ClientBuilder+node"></a>

### clientBuilder.node(url) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 

<a name="ClientBuilder+primaryNode"></a>

### clientBuilder.primaryNode(url, jwt, username, password) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="ClientBuilder+primaryPowNode"></a>

### clientBuilder.primaryPowNode(url, jwt, username, password) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="ClientBuilder+permanode"></a>

### clientBuilder.permanode(url, jwt, username, password) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="ClientBuilder+nodeAuth"></a>

### clientBuilder.nodeAuth(url, jwt, username, password) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="ClientBuilder+nodeSyncInterval"></a>

### clientBuilder.nodeSyncInterval(value) ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="ClientBuilder+nodeSyncDisabled"></a>

### clientBuilder.nodeSyncDisabled() ⇒ [<code>ClientBuilder</code>](#ClientBuilder)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  
<a name="ClientBuilder+quorum"></a>

### clientBuilder.quorum(value)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| value | <code>boolean</code> | 

<a name="ClientBuilder+quorumSize"></a>

### clientBuilder.quorumSize(value)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="ClientBuilder+quorumThreshold"></a>

### clientBuilder.quorumThreshold(value)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="ClientBuilder+tipsInterval"></a>

### clientBuilder.tipsInterval(value)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="ClientBuilder+requestTimeout"></a>

### clientBuilder.requestTimeout(value)
**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="ClientBuilder+build"></a>

### clientBuilder.build() ⇒ [<code>Client</code>](#Client)
Build the client.

**Kind**: instance method of [<code>ClientBuilder</code>](#ClientBuilder)  
<a name="MessageBuilder"></a>

## MessageBuilder
**Kind**: global class  

* [MessageBuilder](#MessageBuilder)
    * _instance_
        * [.index(index)](#MessageBuilder+index) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
        * [.data(data)](#MessageBuilder+data) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
        * [.seed(seed)](#MessageBuilder+seed) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
        * [.accountIndex(account_index)](#MessageBuilder+accountIndex) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
        * [.initialAddressIndex(initial_address_index)](#MessageBuilder+initialAddressIndex) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
        * [.parents(parents)](#MessageBuilder+parents) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
        * [.input(transaction_id, index)](#MessageBuilder+input) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
        * [.inputRange(start, end)](#MessageBuilder+inputRange) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
        * [.output(address, amount)](#MessageBuilder+output) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
        * [.dustAllowanceOutput(address, amount)](#MessageBuilder+dustAllowanceOutput) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
        * [.submit()](#MessageBuilder+submit) ⇒ <code>Promise.&lt;any&gt;</code>
    * _static_
        * [.new(client)](#MessageBuilder.new) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)

<a name="MessageBuilder+index"></a>

### messageBuilder.index(index) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| index | <code>Uint8Array</code> | 

<a name="MessageBuilder+data"></a>

### messageBuilder.data(data) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| data | <code>Uint8Array</code> | 

<a name="MessageBuilder+seed"></a>

### messageBuilder.seed(seed) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| seed | <code>string</code> | 

<a name="MessageBuilder+accountIndex"></a>

### messageBuilder.accountIndex(account_index) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| account_index | <code>number</code> | 

<a name="MessageBuilder+initialAddressIndex"></a>

### messageBuilder.initialAddressIndex(initial_address_index) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| initial_address_index | <code>number</code> | 

<a name="MessageBuilder+parents"></a>

### messageBuilder.parents(parents) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| parents | <code>any</code> | 

<a name="MessageBuilder+input"></a>

### messageBuilder.input(transaction_id, index) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| transaction_id | <code>string</code> | 
| index | <code>number</code> | 

<a name="MessageBuilder+inputRange"></a>

### messageBuilder.inputRange(start, end) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| start | <code>number</code> | 
| end | <code>number</code> | 

<a name="MessageBuilder+output"></a>

### messageBuilder.output(address, amount) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| address | <code>string</code> | 
| amount | <code>BigInt</code> | 

<a name="MessageBuilder+dustAllowanceOutput"></a>

### messageBuilder.dustAllowanceOutput(address, amount) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| address | <code>string</code> | 
| amount | <code>BigInt</code> | 

<a name="MessageBuilder+submit"></a>

### messageBuilder.submit() ⇒ <code>Promise.&lt;any&gt;</code>
Build and sumbit the message.

**Kind**: instance method of [<code>MessageBuilder</code>](#MessageBuilder)  
<a name="MessageBuilder.new"></a>

### MessageBuilder.new(client) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
**Kind**: static method of [<code>MessageBuilder</code>](#MessageBuilder)  

| Param | Type |
| --- | --- |
| client | [<code>Client</code>](#Client) | 

<a name="MessageGetter"></a>

## MessageGetter
**Kind**: global class  

* [MessageGetter](#MessageGetter)
    * _instance_
        * [.index(index)](#MessageGetter+index) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.data(message_id)](#MessageGetter+data) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.raw(message_id)](#MessageGetter+raw) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.children(message_id)](#MessageGetter+children) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.metadata(message_id)](#MessageGetter+metadata) ⇒ <code>Promise.&lt;any&gt;</code>
    * _static_
        * [.new(client)](#MessageGetter.new) ⇒ [<code>MessageGetter</code>](#MessageGetter)

<a name="MessageGetter+index"></a>

### messageGetter.index(index) ⇒ <code>Promise.&lt;any&gt;</code>
Get message ids with an index.

**Kind**: instance method of [<code>MessageGetter</code>](#MessageGetter)  

| Param | Type |
| --- | --- |
| index | <code>Uint8Array</code> | 

<a name="MessageGetter+data"></a>

### messageGetter.data(message_id) ⇒ <code>Promise.&lt;any&gt;</code>
Get a message with the message id.

**Kind**: instance method of [<code>MessageGetter</code>](#MessageGetter)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="MessageGetter+raw"></a>

### messageGetter.raw(message_id) ⇒ <code>Promise.&lt;any&gt;</code>
Get the raw message with the message id.

**Kind**: instance method of [<code>MessageGetter</code>](#MessageGetter)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="MessageGetter+children"></a>

### messageGetter.children(message_id) ⇒ <code>Promise.&lt;any&gt;</code>
Get the childrens of a message with the message id.

**Kind**: instance method of [<code>MessageGetter</code>](#MessageGetter)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="MessageGetter+metadata"></a>

### messageGetter.metadata(message_id) ⇒ <code>Promise.&lt;any&gt;</code>
Get the metadata of a message with the message id.

**Kind**: instance method of [<code>MessageGetter</code>](#MessageGetter)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="MessageGetter.new"></a>

### MessageGetter.new(client) ⇒ [<code>MessageGetter</code>](#MessageGetter)
**Kind**: static method of [<code>MessageGetter</code>](#MessageGetter)  

| Param | Type |
| --- | --- |
| client | [<code>Client</code>](#Client) | 

<a name="start"></a>

## start()
Initializes the console error panic hook for better error messages

**Kind**: global function  
