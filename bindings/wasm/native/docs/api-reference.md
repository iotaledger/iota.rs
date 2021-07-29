## Classes

<dl>
<dt><a href="#AddressGetter">AddressGetter</a></dt>
<dd></dd>
<dt><a href="#BalanceGetter">BalanceGetter</a></dt>
<dd></dd>
<dt><a href="#Client">Client</a></dt>
<dd></dd>
<dt><a href="#ClientBuilder">ClientBuilder</a></dt>
<dd></dd>
<dt><a href="#MessageBuilder">MessageBuilder</a></dt>
<dd></dd>
<dt><a href="#MessageGetter">MessageGetter</a></dt>
<dd></dd>
<dt><a href="#UnspentAddressGetter">UnspentAddressGetter</a></dt>
<dd></dd>
</dl>

## Functions

<dl>
<dt><a href="#start">start()</a></dt>
<dd><p>Initializes the console error panic hook for better error messages</p>
</dd>
</dl>

<a name="AddressGetter"></a>

## AddressGetter
**Kind**: global class  

* [AddressGetter](#AddressGetter)
    * _instance_
        * [.accountIndex(index)](#AddressGetter+accountIndex) ⇒ [<code>AddressGetter</code>](#AddressGetter)
        * [.range(start, end)](#AddressGetter+range) ⇒ [<code>AddressGetter</code>](#AddressGetter)
        * [.bech32Hrp(bech32_hrp)](#AddressGetter+bech32Hrp) ⇒ [<code>AddressGetter</code>](#AddressGetter)
        * [.includeInternal()](#AddressGetter+includeInternal) ⇒ [<code>AddressGetter</code>](#AddressGetter)
        * [.get()](#AddressGetter+get) ⇒ <code>Promise.&lt;any&gt;</code>
    * _static_
        * [.new(client, seed)](#AddressGetter.new) ⇒ [<code>AddressGetter</code>](#AddressGetter)

<a name="AddressGetter+accountIndex"></a>

### addressGetter.accountIndex(index) ⇒ [<code>AddressGetter</code>](#AddressGetter)
Set the account index

**Kind**: instance method of [<code>AddressGetter</code>](#AddressGetter)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="AddressGetter+range"></a>

### addressGetter.range(start, end) ⇒ [<code>AddressGetter</code>](#AddressGetter)
Set the address range

**Kind**: instance method of [<code>AddressGetter</code>](#AddressGetter)  

| Param | Type |
| --- | --- |
| start | <code>number</code> | 
| end | <code>number</code> | 

<a name="AddressGetter+bech32Hrp"></a>

### addressGetter.bech32Hrp(bech32_hrp) ⇒ [<code>AddressGetter</code>](#AddressGetter)
Set the bech32 hrp

**Kind**: instance method of [<code>AddressGetter</code>](#AddressGetter)  

| Param | Type |
| --- | --- |
| bech32_hrp | <code>string</code> | 

<a name="AddressGetter+includeInternal"></a>

### addressGetter.includeInternal() ⇒ [<code>AddressGetter</code>](#AddressGetter)
Include internal addresses

**Kind**: instance method of [<code>AddressGetter</code>](#AddressGetter)  
<a name="AddressGetter+get"></a>

### addressGetter.get() ⇒ <code>Promise.&lt;any&gt;</code>
Get the addresses.

**Kind**: instance method of [<code>AddressGetter</code>](#AddressGetter)  
<a name="AddressGetter.new"></a>

### AddressGetter.new(client, seed) ⇒ [<code>AddressGetter</code>](#AddressGetter)
**Kind**: static method of [<code>AddressGetter</code>](#AddressGetter)  

| Param | Type |
| --- | --- |
| client | [<code>Client</code>](#Client) | 
| seed | <code>string</code> | 

<a name="BalanceGetter"></a>

## BalanceGetter
**Kind**: global class  

* [BalanceGetter](#BalanceGetter)
    * _instance_
        * [.accountIndex(index)](#BalanceGetter+accountIndex) ⇒ [<code>BalanceGetter</code>](#BalanceGetter)
        * [.initialAddressIndex(initial_address_index)](#BalanceGetter+initialAddressIndex) ⇒ [<code>BalanceGetter</code>](#BalanceGetter)
        * [.gap_limit(gap_limit)](#BalanceGetter+gap_limit) ⇒ [<code>BalanceGetter</code>](#BalanceGetter)
        * [.get()](#BalanceGetter+get) ⇒ <code>Promise.&lt;any&gt;</code>
    * _static_
        * [.new(client, seed)](#BalanceGetter.new) ⇒ [<code>BalanceGetter</code>](#BalanceGetter)

<a name="BalanceGetter+accountIndex"></a>

### balanceGetter.accountIndex(index) ⇒ [<code>BalanceGetter</code>](#BalanceGetter)
Sets the account index

**Kind**: instance method of [<code>BalanceGetter</code>](#BalanceGetter)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="BalanceGetter+initialAddressIndex"></a>

### balanceGetter.initialAddressIndex(initial_address_index) ⇒ [<code>BalanceGetter</code>](#BalanceGetter)
Sets the address index from which to start looking for balance

**Kind**: instance method of [<code>BalanceGetter</code>](#BalanceGetter)  

| Param | Type |
| --- | --- |
| initial_address_index | <code>number</code> | 

<a name="BalanceGetter+gap_limit"></a>

### balanceGetter.gap\_limit(gap_limit) ⇒ [<code>BalanceGetter</code>](#BalanceGetter)
Sets the gap limit to specify how many addresses will be checked each round.
If gap_limit amount of addresses in a row have no balance the function will return.

**Kind**: instance method of [<code>BalanceGetter</code>](#BalanceGetter)  

| Param | Type |
| --- | --- |
| gap_limit | <code>number</code> | 

<a name="BalanceGetter+get"></a>

### balanceGetter.get() ⇒ <code>Promise.&lt;any&gt;</code>
Get the balance.

**Kind**: instance method of [<code>BalanceGetter</code>](#BalanceGetter)  
<a name="BalanceGetter.new"></a>

### BalanceGetter.new(client, seed) ⇒ [<code>BalanceGetter</code>](#BalanceGetter)
**Kind**: static method of [<code>BalanceGetter</code>](#BalanceGetter)  

| Param | Type |
| --- | --- |
| client | [<code>Client</code>](#Client) | 
| seed | <code>string</code> | 

<a name="Client"></a>

## Client
**Kind**: global class  

* [Client](#Client)
    * [.message()](#Client+message) ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
    * [.getMessage()](#Client+getMessage) ⇒ [<code>MessageGetter</code>](#MessageGetter)
    * [.getAddresses(seed)](#Client+getAddresses) ⇒ [<code>AddressGetter</code>](#AddressGetter)
    * [.getUnspentAddress(seed)](#Client+getUnspentAddress) ⇒ [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)
    * [.getBalance(seed)](#Client+getBalance) ⇒ [<code>BalanceGetter</code>](#BalanceGetter)
    * [.getInfo()](#Client+getInfo) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.getHealth()](#Client+getHealth) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.getTips()](#Client+getTips) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.getPeers()](#Client+getPeers) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.postMessage(message)](#Client+postMessage) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.retry(message_id)](#Client+retry) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.retryUntilIncluded(message_id, interval, max_attempts)](#Client+retryUntilIncluded) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.reattach(message_id)](#Client+reattach) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.promote(message_id)](#Client+promote) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.consolidateFunds(seed, account_index, start_index, end_index)](#Client+consolidateFunds) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.bech32ToHex(address)](#Client+bech32ToHex) ⇒ <code>string</code>
    * [.hexToBech32(address, bech32)](#Client+hexToBech32) ⇒ <code>Promise.&lt;any&gt;</code>

<a name="Client+message"></a>

### client.message() ⇒ [<code>MessageBuilder</code>](#MessageBuilder)
Send a message to the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getMessage"></a>

### client.getMessage() ⇒ [<code>MessageGetter</code>](#MessageGetter)
Get a message from the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getAddresses"></a>

### client.getAddresses(seed) ⇒ [<code>AddressGetter</code>](#AddressGetter)
Generate addresses.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| seed | <code>string</code> | 

<a name="Client+getUnspentAddress"></a>

### client.getUnspentAddress(seed) ⇒ [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)
Get an unspent address.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| seed | <code>string</code> | 

<a name="Client+getBalance"></a>

### client.getBalance(seed) ⇒ [<code>BalanceGetter</code>](#BalanceGetter)
Get the account balance.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| seed | <code>string</code> | 

<a name="Client+getInfo"></a>

### client.getInfo() ⇒ <code>Promise.&lt;any&gt;</code>
Get the nodeinfo.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getHealth"></a>

### client.getHealth() ⇒ <code>Promise.&lt;any&gt;</code>
Get the node health.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getTips"></a>

### client.getTips() ⇒ <code>Promise.&lt;any&gt;</code>
Get tips.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+getPeers"></a>

### client.getPeers() ⇒ <code>Promise.&lt;any&gt;</code>
Get peers.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+postMessage"></a>

### client.postMessage(message) ⇒ <code>Promise.&lt;any&gt;</code>
Post message.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| message | <code>any</code> | 

<a name="Client+retry"></a>

### client.retry(message_id) ⇒ <code>Promise.&lt;any&gt;</code>
Retries (promotes or reattaches) a message for provided message id. Message should only be
retried only if they are valid and haven't been confirmed for a while.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="Client+retryUntilIncluded"></a>

### client.retryUntilIncluded(message_id, interval, max_attempts) ⇒ <code>Promise.&lt;any&gt;</code>
Only works in browser because of the timeouts
Retries (promotes or reattaches) a message for provided message id until it's included (referenced by a
milestone). Default interval is 5 seconds and max attempts is 10. Returns reattached messages

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 
| interval | <code>BigInt</code> \| <code>undefined</code> | 
| max_attempts | <code>BigInt</code> \| <code>undefined</code> | 

<a name="Client+reattach"></a>

### client.reattach(message_id) ⇒ <code>Promise.&lt;any&gt;</code>
Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
confirmed for a while.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="Client+promote"></a>

### client.promote(message_id) ⇒ <code>Promise.&lt;any&gt;</code>
Promotes a message. The method should validate if a promotion is necessary through get_message. If not, the
method should error out and should not allow unnecessary promotions.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="Client+consolidateFunds"></a>

### client.consolidateFunds(seed, account_index, start_index, end_index) ⇒ <code>Promise.&lt;any&gt;</code>
Only works in browser because of the timeouts
Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
Returns the address to which the funds got consolidated, if any were available

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| seed | <code>string</code> | 
| account_index | <code>number</code> | 
| start_index | <code>number</code> | 
| end_index | <code>number</code> | 

<a name="Client+bech32ToHex"></a>

### client.bech32ToHex(address) ⇒ <code>string</code>
Returns a parsed hex String from bech32.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| address | <code>string</code> | 

<a name="Client+hexToBech32"></a>

### client.hexToBech32(address, bech32) ⇒ <code>Promise.&lt;any&gt;</code>
Returns a parsed bech32 String from hex.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| address | <code>string</code> | 
| bech32 | <code>string</code> \| <code>undefined</code> | 

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

<a name="UnspentAddressGetter"></a>

## UnspentAddressGetter
**Kind**: global class  

* [UnspentAddressGetter](#UnspentAddressGetter)
    * _instance_
        * [.accountIndex(index)](#UnspentAddressGetter+accountIndex) ⇒ [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)
        * [.initialAddressIndex(index)](#UnspentAddressGetter+initialAddressIndex) ⇒ [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)
        * [.get()](#UnspentAddressGetter+get) ⇒ <code>Promise.&lt;any&gt;</code>
    * _static_
        * [.new(client, seed)](#UnspentAddressGetter.new) ⇒ [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)

<a name="UnspentAddressGetter+accountIndex"></a>

### unspentAddressGetter.accountIndex(index) ⇒ [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)
Sets the account index

**Kind**: instance method of [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="UnspentAddressGetter+initialAddressIndex"></a>

### unspentAddressGetter.initialAddressIndex(index) ⇒ [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)
Sets the index of the address to start looking for balance

**Kind**: instance method of [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="UnspentAddressGetter+get"></a>

### unspentAddressGetter.get() ⇒ <code>Promise.&lt;any&gt;</code>
Get an unspent address with its index.

**Kind**: instance method of [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)  
<a name="UnspentAddressGetter.new"></a>

### UnspentAddressGetter.new(client, seed) ⇒ [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)
**Kind**: static method of [<code>UnspentAddressGetter</code>](#UnspentAddressGetter)  

| Param | Type |
| --- | --- |
| client | [<code>Client</code>](#Client) | 
| seed | <code>string</code> | 

<a name="start"></a>

## start()
Initializes the console error panic hook for better error messages

**Kind**: global function  
