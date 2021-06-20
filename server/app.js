const express = require("express")
const app = express();
const puerto = parseInt(process.env.PORT, 10) || 8889;
const Cors = require("cors")
const mongoose = require("mongoose")
const environment = process.env.NODE_ENV
const jwt = require('jsonwebtoken');
var dbLink = new String()
require('dotenv').config()

// Set environment
console.log(`Current environment -> ${environment}`);
if (environment == "production")
	dbLink = "mongodb://roundCross_DB:27017/mongocross"
else 
	dbLink = "mongodb://localhost:27017/mongocross"


// Middlewares
app.use(Cors());
app.use(express.json());
app.use(express.urlencoded({ extended: false }))


// Routes
// app.use("/login", require("./requests/loginRequests"))


// Open port
app.listen(puerto, () => console.log("Listening port " + puerto))


// JWT Authenticate
function validateToken(req, res, next) {
	const token = req.headers["authorization"]
	if (!token)
		return res.status(200).send({ code: "400", status: "Access denied, no authorization token received" });

	 jwt.verify(token, process.env.SECRET, (err, user) => {
		 if (err)
			return res.status(200).send({ code: "400", status: "Access denied, token expired or incorrect" });
		 next()
	 })
}

// DataBase connection
if (environment != "testing") {
	let timeOut = setInterval(() => {
		mongoose.connect(dbLink, { useNewUrlParser: true, useFindAndModify: false }, (err) => {
			if (err) {
				console.log("Encountered an error in Db Connection")
			} else {
				console.log("Succesfully connected with DB");
				clearInterval(timeOut)
			}
		})
	}, 5000);
}

// ++++++++++++++++ HTTP METHODS +++++++++++++++++++ //

app.get("/", (req, res) => {
	res.send("RoundCross server is up and running! :D")
})



module.exports = app