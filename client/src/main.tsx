import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import { Msg } from './entity/msg'
import './index.css'
import { Client } from './net/core'
// @ts-ignore
BigInt.prototype.toJSON = function () {
  return this.toString()
}

const getRandomInt = (min: number, max: number): number => {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min) + min); // The maximum is exclusive and the minimum is inclusive
}


const test1 = async (): Promise<Number> => {
  return new Promise((resolve) => {
    let v = getRandomInt(1000, 2000);
    setTimeout(() => {
      resolve(v)
    }, v)
  })
}

const test2 = async () => {
  let v = await test1()
  console.log(v)
  await test2()
}

test2()

let client = new Client("[::1]:11122", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOjEyMzEyMzEyMywiZXhwIjoxNjc3NTA0MzM0NzcyLCJpYXQiOjE2NzY4OTk1MzQ3NzIsImlzcyI6IlBSSU0iLCJuYmYiOjE2NzY4OTk1MzQ3NzIsInN1YiI6IiJ9.QVvHSHaio7JWNru-IQjrkl5HFDi5pUOMHZFfknJtEZA", "udp", 123123123n, 1)
await client.connect()
await client.send(Msg.text(1n, 2n, 3, "一只猫"))

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
