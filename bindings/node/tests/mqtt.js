const { Client } = require('../lib/')

const client = new Client('http://localhost:14265')
client.subscriber().topic('milestones/latest').topic('milestones/solid').subscribe(console.log)
