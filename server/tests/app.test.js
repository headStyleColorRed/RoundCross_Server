const chai = require("chai")
const chaiHttp = require("chai-http")
const expect = chai.expect
chai.use(chaiHttp)

const app = require("../app.js")
const mongoose = require("mongoose")


describe("App Tests", () => {

    before((done) => {
		mongoose.connect("mongodb://localhost:27017/testmongocross", { useNewUrlParser: true }, (err) => { done() })
      });


	it("Server is up", async () => {
    	let res = await chai.request(app).get('/')
    	expect(res.status).to.equal(200)
	})

    after((done) => {
      return mongoose.disconnect(done);
    });
})