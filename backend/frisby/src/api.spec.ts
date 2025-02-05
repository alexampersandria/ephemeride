import 'dotenv'
import { generateID } from './util/generateID'
import { get, post } from 'frisby'
import { env } from './env'

const url = env.URL

it("'/v1' responds with 200", (done) => {
  get(`${url}/v1`).expect('status', 200).done(done)
})

const user = {
  email: `${generateID(10)}@example.com`,
  password: generateID(10),
  name: `${generateID(5)} ${generateID(10)}`,
}

describe('User', () => {
  it('POST /v1/user', (done) => {
    post(`${url}/v1/user`, user).expect('status', 200).done(done)
  })
})
