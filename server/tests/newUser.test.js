const chai = require("chai");
const chaiHttp = require("chai-http");
const expect = chai.expect;
chai.use(chaiHttp);

const app = require("../app.js");
const mongoose = require("mongoose");

describe("New User Queries", () => {
	before((done) => {
		mongoose.connect(
			"mongodb://localhost:27017/testmongocross",
			{ useNewUrlParser: true, useFindAndModify: false },
			(err) => {
				done();
			}
		);
	});

	describe("New User", () => {
		it("New user without parameters", async () => {
			let res = await chai.request(app).post("/user/new_user").send({});
			expect(res.status).to.equal(200);
			expect(res.body.code).to.equal("5000");
			expect(res.body.status).to.equal(
				"Fields missing: [email,username]"
			);
		});

		it("New user without email", async () => {
			let res = await chai.request(app).post("/user/new_user").send({
				username: "Michael Scott",
			});
			expect(res.status).to.equal(200);
			expect(res.body.code).to.equal("5000");
			expect(res.body.status).to.equal("Fields missing: [email]");
		});

		it("New user without userName", async () => {
			let res = await chai.request(app).post("/user/new_user").send({
				email: "michaelscott@dundermifflin.com",
			});
			expect(res.status).to.equal(200);
			expect(res.body.code).to.equal("5000");
			expect(res.body.status).to.equal("Fields missing: [username]");
		});

		it("New User", async () => {
			let res = await chai.request(app).post("/user/new_user").send({
				email: "michaelscott@dundermifflin.com",
				username: "Michael Scott",
			});
			expect(res.status).to.equal(200);
			expect(res.body.code).to.equal("200");
			expect(res.body.status).to.equal("Created new user Succesfully");
		});

		it("Register already registered user", async () => {
			let res = await chai.request(app).post("/user/new_user").send({
				email: "michaelscott@dundermifflin.com",
				username: "Michael Scott",
			});
			expect(res.status).to.equal(200);
			expect(res.body.code).to.equal("400");
			expect(res.body.status).to.equal("User already exists");
		});
	});

    describe("Onboarding", () => {
		it("Onboarding data without parameters", async () => {
			let res = await chai.request(app).post("/user/new_user_onboarding").send({});
			expect(res.status).to.equal(200);
			expect(res.body.code).to.equal("5000");
			expect(res.body.status).to.equal(
				"Fields missing: [email,nickName,country,bikingModality]"
			);
		});

        it("Onboarding with biking modality as string", async () => {
			let res = await chai.request(app).post("/user/new_user_onboarding").send({
				email: "michaelscott@dundermifflin.com",
                nickName: "ParkourMichael",
                country: "USA",
                bikingModality: "lol"
			});
			expect(res.status).to.equal(200);
			expect(res.body.code).to.equal("400");
			expect(res.body.status).to.equal("Error updating new user fields");
		});

		it("Onboarding data", async () => {
			let res = await chai.request(app).post("/user/new_user_onboarding").send({
				email: "michaelscott@dundermifflin.com",
                nickName: "ParkourMichael",
                country: "USA",
                bikingModality: 0
			});
			expect(res.status).to.equal(200);
			expect(res.body.code).to.equal("200");
			expect(res.body.status).to.equal("Added new user data to user");
		});
	});

	after((done) => {
		chai.request(app)
			.get("/deleteUsers")
			.then(() => {
				mongoose.disconnect(done);
			});
	});
});
