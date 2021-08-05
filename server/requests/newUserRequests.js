const express = require("express")
const router = express.Router()

// Modules
const User = require("../mongoDB/userModel.js")
const ValidationManager = require("../tools/validation.js")
const RequestManager = require("./requestManager")


router.post("/new_user", async (req, res) => {
	let body = req.body

    // Validation
    let validationResult = ValidationManager.validateDataFields( body, ["email"], "creating new user" );
    if (validationResult.isError)
        return res.status(200).send({ status: validationResult.error, smessage: validationResult.message });

	// Create new user
	const user = new User({email: body.email, username: body.username});

	try {
		await user.save().catch((err) => { throw err })
	} catch (err) {
		return res.status(200).send({ status: "400", message: err.code == 11000 ? "User already exists" : err}) 
	}

	res.status(200).send({ status: "200", message: "Created new user Succesfully"})
});


router.post("/new_user_onboarding", async (req, res) => {
	let body = req.body

    // Validation
    let validationResult = ValidationManager.validateDataFields( body, ["email", "userName", "country", "bikingModality"], "new user onboarding" );
    if (validationResult.isError)
        return res.status(200).send({ status: validationResult.error, message: validationResult.message });

	// New User data
	let newUser = {
		userName: body.userName,
		country: body.country,
		bikingModality: body.bikingModality
	}

	// Add new parameters
	try {
		await RequestManager.updateUserField(body.email, newUser).catch((err) => { throw err })
	} catch (err) {
		return res.status(200).send({ status: "400", message: "Error updating new user fields"}) 
	}

	res.status(200).send({ status: "200", message: "Added new user data to user"})
});



module.exports = router;