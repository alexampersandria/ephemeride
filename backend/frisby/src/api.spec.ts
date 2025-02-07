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
  it('POST /v1/user 201', (done) => {
    post(`${url}/v1/user`, user).expect('status', 201).done(done)
  })

  let session: String | null = null

  it('POST /v1/auth 201 and return session', (done) => {
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

  it('POST /v1/auth 401 wrong password', (done) => {
    const authUser = {
      email: user.email,
      password: 'wrongpassword',
    }

    post(`${url}/v1/auth`, authUser).expect('status', 401).done(done)
  })

  it('POST /v1/auth 400 invalid email', (done) => {
    const authUser = {
      email: 'invalidemail',
      password: 'wrongpassword',
    }

    post(`${url}/v1/auth`, authUser).expect('status', 400).done(done)
  })

  it('GET /v1/user 200', (done) => {
    get(`${url}/v1/user`, { headers: { Authorization: `Bearer ${session}` } })
      .expect('status', 200)
      .done(done)
  })

  it('GET /v1/user 401 without auth', (done) => {
    get(`${url}/v1/user`).expect('status', 401).done(done)
  })
})
