# Solana escrow payment （SEP）

# Introduction

 

Solana is a high-performance blockchain technology, known for its operating speed and efficiency, making it an ideal choice for processing payment transactions. Due to its unique design and architecture, Solana can handle a large number of transactions while maintaining a high level of security and stability. Therefore, whether for individual users or corporate customers, Solana can provide efficient and reliable payment solutions.

![Untitled](https://prod-files-secure.s3.us-west-2.amazonaws.com/36958754-f0a0-4bde-9a66-0691b9f754c9/beadf352-8f26-4d54-8757-044983f7eee1/Untitled.png)

Our current payment method is a standard transfer transaction, which moves money from one account to another. Please note, once initiated, this process is irreversible and cannot be cancelled. This is worth considering to avoid any errors. Additionally, this method does not guarantee fund security, which could pose risks if the safety of your money is a priority.

![Untitled](https://prod-files-secure.s3.us-west-2.amazonaws.com/36958754-f0a0-4bde-9a66-0691b9f754c9/fd4765a0-c743-4bc6-8a03-e048d3603827/Untitled.png)

We have decided to implement a secure transaction method with a confirmation mechanism. The core of this transaction method is to perform multiple confirmations before officially transferring funds to the trading counterparty. This method greatly enhances the security of transactions, helps prevent fraudulent behavior, and ensures that the interests of all participants are protected. In every step of the transaction, the confirmation mechanism plays a key role, allowing each transaction member sufficient time to verify the transaction details to ensure the safety of funds. This method not only improves the transparency of transactions, but also increases the efficiency of transactions.

In the future, completing a payment will require three steps.

![Untitled](https://prod-files-secure.s3.us-west-2.amazonaws.com/36958754-f0a0-4bde-9a66-0691b9f754c9/e5587be4-ea0c-4f51-9a2d-befc5c9f2270/Untitled.png)

Step 1:   

In this payment processing procedure, the user first needs to deposit a specified amount of funds into a dedicated program provided by the system. This program is a highly secure software application designed specifically for handling and managing deposit and withdrawal operations. After the deposit is made, the program automatically executes a pre-set event notification mechanism, which is designed to trigger and execute upon successful deposit of funds.

The notification event itself is a piece of automated code that sends confirmation information to the payee based on built-in logic and parameters. This information may include details of the transaction such as the deposit amount, transaction timestamp, transaction ID, and any additional information defined by the user. The event can be a simple status update, informing the payee that the funds are ready, or it can be a more complex process of information transfer that may involve interacting with the payee's program interface to ensure the payee can receive notifications in real-time within their system.

The entire process is aimed at ensuring the transparency and efficiency of the transaction, while also ensuring that all parties involved can receive updates on the transaction status immediately.

 

![Untitled](https://prod-files-secure.s3.us-west-2.amazonaws.com/36958754-f0a0-4bde-9a66-0691b9f754c9/3b49e4c2-cf03-42ba-ae25-8ed09011db05/Untitled.png)

Step 2: Upon confirmation of payment by the payee, the automated system updates the transaction status and initiates the shipping process, which includes order processing, logistics arrangement, and sending shipment notifications to the buyer, with all actions logged in the system for tracking and auditing purposes.

![Untitled](https://prod-files-secure.s3.us-west-2.amazonaws.com/36958754-f0a0-4bde-9a66-0691b9f754c9/1e428e36-8eea-4e28-8a38-5b0202fc785e/Untitled.png)

Step 3: In an e-commerce system, after the user clicks the "Confirm Receipt" button, the system will verify the validity of the operation and update the order status to "Received". Subsequently, the process to trigger payment to the seller is initiated, and the details of the operation are recorded in logs for post-transaction audit.

If any party involved in the transaction wishes to cancel it during the process, they can submit a request for cancellation. The transaction can be cancelled as long as the opposing party agrees to this request. This mechanism ensures flexibility and safeguards the interests of both parties. Additionally, the system records the cancellation, maintaining the process's transparency and fairness.

In case of a dispute, parties involved may submit an arbitration request to the Project Management Committee. The committee will review the dispute and make a determination on whether to terminate or consider the transaction complete, ensuring fairness and transparency of the process.

And if there is a timeout unconfirmed situation, the transaction process will be automatically confirmed.
