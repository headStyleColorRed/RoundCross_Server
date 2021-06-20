const express = require("express")
const router = express.Router()

// Modules
const User = require("../mongoDB/userModel.js")
const ValidationManager = require("../tools/validation.js")


router.post("/new_user", async (req, res) => {
	let body = req.body

    // Validation
    let validationResult = ValidationManager.validateDataFields( body, ["email", "username"], "creating new user" );
    if (validationResult.isError)
        return res.status(200).send({ code: validationResult.error, status: validationResult.message });

	// Create new user
	const user = new User({email: body.email, username: body.username});

	try {
		await user.save().catch((err) => { throw err })
	} catch (err) {
		return res.status(200).send({ code: "400", status: err.code == 11000 ? "User already exists" : err}) 
	}

	res.status(200).send({ code: "200", status: "Created new user Succesfully"})
});




module.exports = router;