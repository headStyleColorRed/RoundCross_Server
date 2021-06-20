// Modules
const User = require("../mongoDB/userModel.js")

async function updateUserField(userEmail, fieldToModifyObject) {
  let promise = new Promise((resolve, reject) => {
    User.updateOne({ email: userEmail }, fieldToModifyObject)
      .then((res) => {
          resolve(res)
      })
      .catch((err) => {
          reject(err)
      });
  });

  let result = await promise;
  return result;
}

module.exports = {
  updateUserField,
};