const { ClientBuilder } = require('../lib/')

const client = new ClientBuilder().node('http://localhost:14265').build()
client.subscriber().topic('milestones/latest').topic('milestones/solid').subscribe(console.log)
