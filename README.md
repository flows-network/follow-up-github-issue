# <p align="center">ChatGPT/4 code reviewer for Github Issue</p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/new">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](#deploy-the-pr-summary-app-for-your-github-repo), and you will get a Slack 🤖 to reminder the open source maintainers the GitHub issues they don't respond in time. It helps busy open source contributors respond the issues from community! 

## How it works

This flow function (or 🤖) will be triggered when a new GitHub issue raising in the designated GitHub repo, but don't get any response (i.e comment, adding a lable, add an assignee, comment with emoji) after the designated date. The flow function will send the issue link as a message to the designated slack channel.  The flow functions are written in Rust and run in hosted [WasmEdge Runtimes](https://github.com/wasmedge) on [flows.network](https://flows.network/).

The orginal 

### Deploy your own code review bot in 3 simple steps

1. Fork this repo to your own GitHub account. It contains the source code for the Slack bot.
2. Import the forked repo on [flows.network](https://flows.network/) as a flow function.
3. Connect the flow function to the GitHub repo you want to deploy the bot on (using the [flows.network](https://flows.network/) UI). 


## Prerequisites

You will need to sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

## Deploy the GitHub issue reminder 🤖 onto your Slack channel

The 🤖 is designed to run on [flows.network](https://flows.network/), a serverless platform for SaaS and AI automation.

### 1 Fork this repo

Fork [this repo](https://github.com/flows-network/follow-up-github-issuey) into your own GitHub account.

### 2 Deploy the bot's source code on flow.network

Go to [flows.network](https://flows.network/) to deploy your own flow function (🤖) from the forked source code.

1. Click on the "Create a Flow" button to start.
2. Authenticate the [flows.network](https://flows.network/) to access the `follow-up-github-issuey` repo you just forked. **NOTE: This is NOT the repo you want to install the bot on.**

<img width="940" alt="image" src="https://user-images.githubusercontent.com/45785633/235133980-074a9c1c-533c-4671-9fc6-b316c0f273b8.png">

3. Click on the "Advanced" link to see more settings. Fill in the following environment variables. 

> The 5 variables below are defined in the flow function's Rust source code. You can assign their values in the source code in your fork directly and skip the steps below.

* `login`: Your personal GitHub id. The GitHub app will act as you when posting reviews.
* `owner`: GitHub org for the repo *you want to monitor the status of GitHub issue*.
* `repo` : GitHub repo *you want tomonitor the status of GitHub issues*.
* `team`: Slack workspace you want receive the reminder messages.
* `channel`: Slack channel you want receive the reminder messages

> Let's see an example. You forked the flow function source code to `my-name/github-pr-summary` and would like to deploy the bot to summarize PRs on `my-company/work-project` repo. Here `login = my-name`, `owner = my-company` and `repo = work-project`.

<img width="880" alt="image" src="https://user-images.githubusercontent.com/45785633/235134820-e81a2e38-587d-46b2-98c4-3909beb74708.png">

4. Click on the Deploy button.

### 3 Configure integrations

After that, [flows.network](https://flows.network/) will direct you to configure the external services required by your flow function 🤖.

<img width="882" alt="image" src="https://user-images.githubusercontent.com/45785633/235135204-f9c000e6-395f-49be-8925-d7daf53af505.png">

For this flow function, we need to configure two integrations.

1. Click on the "Connect" or "+ Add new authentication" button to give the function access to the Slack workspace. That is to give access to the `team/channel` in the environment variables. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to the channel. This should be a public channel.


2. Click on the "Connect" or "+ Add new authentication" button to give the function access to the GitHub repo. That is to give access to the `owner/repo` in the environment variables. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to the repo.

After that, click on the "Check" button to go to the flow details page. As soon as the flow's status became `running`, the bot will send you the issues you forgot to respond. Then you won't miss any issues!

<img width="1140" alt="image" src="https://user-images.githubusercontent.com/45785633/235135643-cd5dad5d-70fa-4f1b-8204-dbc587044e08.png">




