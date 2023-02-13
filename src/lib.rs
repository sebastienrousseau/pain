// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library to manage ISO 20022 pain.001 payment initiation messages widely used for corporate to bank payments.
//!
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-pain.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/pain.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/pain)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.2-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/pain)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/pain)
//! [![License](https://img.shields.io/crates/l/pain.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! Payment Initiation (PI) is a library that provides a set of structs
//! and enums that can be used to serialize and deserialize SEPA payment
//! information using the `serde` library.
//!
//! The structs include Payment, OriginalGroupInfoAndStatus, PaymentInfo,
//! PaymentTypeInfo, ServiceLevel, Debtor, PostalAddress, DebtorAccount,
//! AccountId, DebtorAgent, FinancialInstitutionId,
//! CreditTransferTransactionInfo, PaymentId, Amount, InstructedAmount,
//! Creditor, CreditorAccount, RemittanceInfo, and CreditorAgent.
//!
//! The enum is PaymentFormat and has four variants: Pain001_001_02,
//! Pain001_001_03, Pain001_001_04, and Pain001_001_05.
//!
//!
//! ## PaymentFormat
//!
//! The `PaymentFormat` enum defines the different formats of payment supported by the system.
//!
//! ## Payment
//!
//! The `Payment` struct is the main data structure used to represent a payment. It contains the following fields:
//!
//! - `payment_format`: The format of the payment, represented by the `PaymentFormat` enum.
//! - `original_group_info_and_status`: Information about the original group and its status, represented by the `OriginalGroupInfoAndStatus` struct.
//! - `payment_info`: Information about the payment, represented by the `PaymentInfo` struct.
//!
//! ## OriginalGroupInfoAndStatus
//!
//! The `OriginalGroupInfoAndStatus` struct contains information about the original group and its status. It has the following fields:
//!
//! - `original_message_id`: The original message ID.
//! - `original_message_name_id`: The original message name ID.
//! - `group_status`: The status of the group.
//!
//! ## PaymentInfo
//!
//! The `PaymentInfo` struct contains information about the payment. It has the following fields:
//!
//! - `payment_info_id`: The ID of the payment information.
//! - `payment_method`: The method used for the payment.
//! - `number_of_transactions`: The number of transactions in the payment.
//! - `control_sum`: The control sum of the payment.
//! - `payment_type_info`: Information about the type of payment, represented by the `PaymentTypeInfo` struct.
//! - `requested_execution_date`: The requested execution date of the payment.
//! - `debtor`: Information about the debtor, represented by the `Debtor` struct.
//! - `debtor_account`: Information about the debtor's account, represented by the `DebtorAccount` struct.
//! - `debtor_agent`: Information about the debtor's agent, represented by the `DebtorAgent` struct.
//! - `credit_transfer_transaction_info`: Information about the credit transfer transactions, represented by the `CreditTransferTransactionInfo` struct.
//!
//! ## PaymentTypeInfo
//!
//! The `PaymentTypeInfo` struct contains information about the type of payment. It has the following fields:
//!
//! - `service_level`: The service level of the payment, represented by the `ServiceLevel` struct.
//! - `local_instrument`: The local instrument used for the payment.
//! - `payment_type_id`: The ID of the payment type.
//!
//! ## ServiceLevel
//!
//! The `ServiceLevel` struct contains information about the service level of a payment. It has the following field:
//!
//! - `code`: The code representing the service level.
//!
//! ## Debtor
//! The `Debtor` struct contains information about the debtor. It has the following fields:
//!
//! - `name`: The name of the debtor.
//! - `postal_address`: The postal address of the debtor, represented by the `PostalAddress` struct.
//!
//! ## PostalAddress
//!
//! The `PostalAddress` struct contains information about a postal address. It has the following fields:
//!
//! - `country`: The country of the address.
//! - `address_line`: The lines of the address.
//! //!
//! # DebtorAccount
//!
//! The `DebtorAccount` struct contains information about the debtor's account. It has the following fields:
//!
//! - `id`: The ID of the account, represented by the `AccountId` struct.
//!
//! # AccountId
//!
//! The `AccountId` struct contains information about an account ID. It has the following fields:
//!
//! - `IBAN`: The International Bank Account Number (IBAN) of the account.
//! - `other`: Information about other types of accounts, represented by the `OtherAccountId` struct.
//!
//! # OtherAccountId
//!
//! The `OtherAccountId` struct contains information about other types of accounts. It has the following fields:
//!
//! - `id`: The ID of the account.
//! - `issuer`: The issuer of the account.
//!
//! # DebtorAgent
//!
//! The `DebtorAgent` struct contains information about the debtor's agent. It has the following fields:
//!
//! - `financial_institution_id`: The ID of the financial institution.
//!
//! # CreditTransferTransactionInfo
//!
//! The `CreditTransferTransactionInfo` struct contains information about the credit transfer transactions. It has the following fields:
//!
//! - `amount`: The amount of the transaction.
//! - `currency`: The currency of the transaction.
//! - `remittance_information`: The remittance information of the transaction.
//! - `creditor`: Information about the creditor, represented by the `Creditor` struct.
//! - `creditor_account`: Information about the creditor's account, represented by the `CreditorAccount` struct.
//! - `creditor_agent`: Information about the creditor's agent, represented by the `CreditorAgent` struct.
//! - `ultimate_creditor`: Information about the ultimate creditor, represented by the `UltimateCreditor` struct.
//!
//! # Creditor
//!
//! The `Creditor` struct contains information about the creditor. It has the following fields:
//!
//! - `name`: The name of the creditor.
//! - `postal_address`: The postal address of the creditor, represented by the `PostalAddress` struct.
//!
//! # CreditorAccount
//!
//! The `CreditorAccount` struct contains information about the creditor's account. It has the following fields:
//!
//! - `id`: The ID of the account, represented by the `AccountId` struct.
//!
//! # CreditorAgent
//!
//! The `CreditorAgent` struct contains information about the creditor's agent. It has the following fields:
//!
//! - `financial_institution_id`: The ID of the financial institution.
//!
//! # UltimateCreditor
//!
//! The `UltimateCreditor` struct contains information about the ultimate creditor. It has the following fields:
//!
//! - `name`: The name of the ultimate creditor.
//! - `postal_address`: The postal address of the ultimate creditor, represented by the `PostalAddress` struct.
//!
//! ## Usage
//!
//! - [`serde`][]: Enable serialization/deserialization via serde
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!
// #![cfg_attr(feature = "bench", feature(test))]
// #![deny(dead_code)]
// #![deny(missing_debug_implementations)]
// #![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/pain/icons/ico-pain.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/pain/icons/ico-pain.svg",
    html_root_url = "https://docs.rs/pain"
)]
#![crate_name = "pain"]
#![crate_type = "lib"]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub enum PaymentFormat {
    Pain001_001_02,
    Pain001_001_03,
    Pain001_001_04,
    Pain001_001_05,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename = "payment")]
pub struct Payment<T> {
    pub payment_format: T,

    #[serde(rename = "OrgnlGrpInfAndSts")]
    pub original_group_info_and_status: OriginalGroupInfoAndStatus,

    #[serde(rename = "PmtInf")]
    pub payment_info: PaymentInfo,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OriginalGroupInfoAndStatus {
    #[serde(rename = "OrgnlMsgId")]
    pub original_message_id: String,

    #[serde(rename = "OrgnlMsgNmId")]
    pub original_message_name_id: String,

    #[serde(rename = "GrpSts")]
    pub group_status: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentInfo {
    #[serde(rename = "PmtInfId")]
    pub payment_info_id: String,

    #[serde(rename = "PmtMtd")]
    pub payment_method: String,

    #[serde(rename = "NbOfTxs")]
    pub number_of_transactions: String,

    #[serde(rename = "CtrlSum")]
    pub control_sum: String,

    #[serde(rename = "PmtTpInf")]
    pub payment_type_info: PaymentTypeInfo,

    #[serde(rename = "ReqdExctnDt")]
    pub requested_execution_date: String,

    #[serde(rename = "Dbtr")]
    pub debtor: Debtor,

    #[serde(rename = "DbtrAcct")]
    pub debtor_account: DebtorAccount,

    #[serde(rename = "DbtrAgt")]
    pub debtor_agent: DebtorAgent,

    #[serde(rename = "CtrTxsInf")]
    pub credit_transfer_transaction_info: Vec<CreditTransferTransactionInfo>,
    pub requested_amount: Amount,

    #[serde(rename = "PmtId")]
    pub payment_id: String,

    #[serde(rename = "PmtTp")]
    pub payment_type: String,

    #[serde(rename = "PmtPrty")]
    pub payment_type_description: String,

    #[serde(rename = "PmtInstrm")]
    pub payment_type_indicator: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentTypeInfo {
    #[serde(rename = "SvcLvl")]
    pub service_level: ServiceLevel,

    #[serde(rename = "LclInstrm")]
    pub local_instrument: String,
    #[serde(rename = "PmtId")]
    pub payment_type_id: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ServiceLevel {
    #[serde(rename = "Cd")]
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Debtor {
    #[serde(rename = "Nm")]
    pub name: String,

    #[serde(rename = "PstlAdr")]
    pub postal_address: PostalAddress,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PostalAddress {
    #[serde(rename = "Ctry")]
    pub country: String,

    #[serde(rename = "AdrLine")]
    pub address_line: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DebtorAccount {
    #[serde(rename = "Id")]
    pub id: AccountId,
    #[serde(rename = "Ccy")]
    pub currency: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountId {
    #[serde(rename = "IBAN")]
    pub iban: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DebtorAgent {
    #[serde(rename = "FinInstnId")]
    pub financial_institution_id: FinancialInstitutionId,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FinancialInstitutionId {
    #[serde(rename = "BIC")]
    pub bic: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreditTransferTransactionInfo {
    #[serde(rename = "PmtId")]
    pub payment_id: PaymentId,
    #[serde(rename = "Amt")]
    pub amount: Amount,

    #[serde(rename = "CdtrAgt")]
    pub creditor_agent: CreditorAgent,

    #[serde(rename = "Cdtr")]
    pub creditor: Creditor,

    #[serde(rename = "CdtrAcct")]
    pub creditor_account: CreditorAccount,

    #[serde(rename = "RmtInf")]
    pub remittance_info: RemittanceInfo,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentId {
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Amount {
    #[serde(rename = "InstdAmt")]
    pub instructed_amount: InstructedAmount,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct InstructedAmount {
    #[serde(rename = "Ccy")]
    pub currency: String,

    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Creditor {
    #[serde(rename = "Nm")]
    pub name: String,

    #[serde(rename = "PstlAdr")]
    pub postal_address: PostalAddress,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreditorAccount {
    #[serde(rename = "Id")]
    pub id: AccountId,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RemittanceInfo {
    #[serde(rename = "Ustrd")]
    pub unstructured: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreditorAgent {
    #[serde(rename = "FinInstnId")]
    pub financial_institution_id: FinancialInstitutionId,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DebtorTransactionInfo {
    #[serde(rename = "PmtId")]
    pub payment_id: PaymentId,
    #[serde(rename = "Amt")]
    pub amount: Amount,
    #[serde(rename = "CdtrAgt")]
    pub creditor_agent: CreditorAgent,
    #[serde(rename = "Cdtr")]
    pub creditor: Creditor,
    #[serde(rename = "CdtrAcct")]
    pub creditor_account: CreditorAccount,
    #[serde(rename = "RmtInf")]
    pub remittance_info: RemittanceInfo,
}

impl std::fmt::Display for PaymentFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PaymentFormat::Pain001_001_02 => write!(f, "pain.001.001.02"),
            PaymentFormat::Pain001_001_03 => write!(f, "pain.001.001.03"),
            PaymentFormat::Pain001_001_04 => write!(f, "pain.001.001.04"),
            PaymentFormat::Pain001_001_05 => write!(f, "pain.001.001.05"),
        }
    }
}
