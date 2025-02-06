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
  let userId: String | null = null

  it('POST /v1/user', (done) => {
    post(`${url}/v1/user`, user)
      .expect('status', 201)
      .then((res) => {
        userId = res.json.user_id
      })
      .done(done)
  })

  let session: String | null = null

  it('POST /v1/auth', (done) => {
    const authUser = {
      email: user.email,
      password: user.password,
    }

    post(`${url}/v1/auth`, authUser)
      .expect('status', 201)
      .then((res) => {
        session = res.json.id
      })
      .done(done)
  })

  it('GET /v1/user/:id', (done) => {
    get(`${url}/v1/user/${userId}`).expect('status', 200).done(done)
  })

  it('GET /v1/user', (done) => {
    get(`${url}/v1/user`, { headers: { Authorization: `Bearer ${session}` } })
      .expect('status', 200)
      .done(done)
  })
})
