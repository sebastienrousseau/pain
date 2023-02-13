#[cfg(test)]
mod tests {
    extern crate pain;
    use pain::*;

    extern crate serde;
    extern crate serde_xml_rs;

    #[test]
    fn test_payment_format() {
        let payment_format_02 = PaymentFormat::Pain001_001_02;
        assert_eq!(payment_format_02.to_string(), "pain.001.001.02");

        let payment_format_03 = PaymentFormat::Pain001_001_03;
        assert_eq!(payment_format_03.to_string(), "pain.001.001.03");

        let payment_format_04 = PaymentFormat::Pain001_001_04;
        assert_eq!(payment_format_04.to_string(), "pain.001.001.04");

        let payment_format_05 = PaymentFormat::Pain001_001_05;
        assert_eq!(payment_format_05.to_string(), "pain.001.001.05");
    }

    #[test]
    fn test_payment() {
        let payment = Payment {
            payment_format: "SEPA".to_string(),
            original_group_info_and_status: OriginalGroupInfoAndStatus {
                original_message_id: "original_message_id".to_string(),
                original_message_name_id: "original_message_name_id".to_string(),
                group_status: "group_status".to_string(),
            },
            payment_info: PaymentInfo {
                payment_id: "payment_id".to_string(),
                payment_type: "payment_type".to_string(),
                payment_type_description: "payment_type_description".to_string(),
                payment_info_id: "payment_info_id".to_string(),
                payment_type_indicator: "payment_type_indicator".to_string(),
                payment_method: "payment_method".to_string(),
                number_of_transactions: "10".to_string(),
                control_sum: "100".to_string(),
                payment_type_info: PaymentTypeInfo {
                    service_level: ServiceLevel {
                        code: "SEPA".to_string(),
                    },
                    local_instrument: "local_instrument".to_string(),
                    payment_type_id: "payment_type_id".to_string(),
                },
                requested_execution_date: "2023-01-01".to_string(),
                debtor: Debtor {
                    name: "debtor_name".to_string(),
                    postal_address: PostalAddress {
                        country: "debtor_country".to_string(),
                        address_line: vec!["debtor_address_line_1".to_string()],
                    },
                },
                debtor_account: DebtorAccount {
                    id: AccountId {
                        iban: "debtor_iban".to_string(),
                    },
                    currency: "EUR".to_string(),
                },
                debtor_agent: DebtorAgent {
                    financial_institution_id: FinancialInstitutionId {
                        bic: "debtor_bic".to_string(),
                    },
                },
                credit_transfer_transaction_info: vec![CreditTransferTransactionInfo {
                    payment_id: PaymentId {
                        end_to_end_id: "end_to_end_id".to_string(),
                    },
                    amount: Amount {
                        instructed_amount: InstructedAmount {
                            currency: "EUR".to_string(),
                            value: "100".to_string(),
                        },
                    },
                    creditor_agent: CreditorAgent {
                        financial_institution_id: FinancialInstitutionId {
                            bic: "creditor_agent_bic".to_string(),
                        },
                    },
                    creditor: Creditor {
                        name: "creditor_name".to_string(),
                        postal_address: PostalAddress {
                            country: "creditor_country".to_string(),
                            address_line: vec!["creditor_address_line_1".to_string()],
                        },
                    },
                    creditor_account: CreditorAccount {
                        id: AccountId {
                            iban: "creditor_iban".to_string(),
                        },
                    },
                    remittance_info: RemittanceInfo {
                        unstructured: "remittance_info".to_string(),
                    },
                }],
                requested_amount: Amount {
                    instructed_amount: InstructedAmount {
                        currency: "EUR".to_string(),
                        value: "100".to_string(),
                    },
                },
            },
        };
        assert_eq!(payment.payment_format, "SEPA");
        assert_eq!(
            payment.payment_info,
            PaymentInfo {
                requested_amount: Amount {
                    instructed_amount: InstructedAmount {
                        currency: "EUR".to_string(),
                        value: "100".to_string(),
                    },
                },
                payment_id: "payment_id".to_string(),
                payment_type: "payment_type".to_string(),
                payment_type_description: "payment_type_description".to_string(),
                payment_type_indicator: "payment_type_indicator".to_string(),
                credit_transfer_transaction_info: vec![CreditTransferTransactionInfo {
                    payment_id: PaymentId {
                        end_to_end_id: "end_to_end_id".to_string(),
                    },
                    amount: Amount {
                        instructed_amount: InstructedAmount {
                            currency: "EUR".to_string(),
                            value: "100".to_string(),
                        },
                    },
                    creditor_agent: CreditorAgent {
                        financial_institution_id: FinancialInstitutionId {
                            bic: "creditor_agent_bic".to_string(),
                        },
                    },
                    creditor: Creditor {
                        name: "creditor_name".to_string(),
                        postal_address: PostalAddress {
                            country: "creditor_country".to_string(),
                            address_line: vec!["creditor_address_line_1".to_string()],
                        },
                    },
                    creditor_account: CreditorAccount {
                        id: AccountId {
                            iban: "creditor_iban".to_string(),
                        },
                    },
                    remittance_info: RemittanceInfo {
                        unstructured: "remittance_info".to_string(),
                    },
                }],
                debtor_account: DebtorAccount {
                    id: AccountId {
                        iban: "debtor_iban".to_string(),
                    },
                    currency: "EUR".to_string(),
                },
                debtor_agent: DebtorAgent {
                    financial_institution_id: FinancialInstitutionId {
                        bic: "debtor_bic".to_string(),
                    },
                },
                payment_info_id: "payment_info_id".to_string(),
                payment_method: "payment_method".to_string(),
                number_of_transactions: "10".to_string(),
                control_sum: "100".to_string(),
                payment_type_info: PaymentTypeInfo {
                    service_level: ServiceLevel {
                        code: "SEPA".to_string(),
                    },
                    local_instrument: "local_instrument".to_string(),
                    payment_type_id: "payment_type_id".to_string(),
                },
                requested_execution_date: "2023-01-01".to_string(),
                debtor: Debtor {
                    name: "debtor_name".to_string(),
                    postal_address: PostalAddress {
                        country: "debtor_country".to_string(),
                        address_line: vec!["debtor_address_line_1".to_string()],
                    },
                },
            }
        );
    }

    #[test]
    fn test_payment_info() {
        let payment_info = PaymentInfo {
            payment_id: "payment_id".to_string(),
            payment_info_id: "payment_info_id".to_string(),
            payment_type_indicator: "payment_type_indicator".to_string(),
            payment_type_description: "payment_type_description".to_string(),
            payment_method: "payment_method".to_string(),
            payment_type: "payment_type".to_string(),
            payment_type_info: PaymentTypeInfo {
                service_level: ServiceLevel {
                    code: "SEPA".to_string(),
                },
                local_instrument: "local_instrument".to_string(),
                payment_type_id: "payment_type_id".to_string(),
            },
            number_of_transactions: "10".to_string(),
            control_sum: "100".to_string(),
            requested_amount: Amount {
                instructed_amount: InstructedAmount {
                    currency: "EUR".to_string(),
                    value: "100".to_string(),
                },
            },
            requested_execution_date: "2023-01-01".to_string(),
            debtor: Debtor {
                name: "debtor_name".to_string(),
                postal_address: PostalAddress {
                    country: "debtor_country".to_string(),
                    address_line: vec!["debtor_address_line_1".to_string()],
                },
            },
            debtor_account: DebtorAccount {
                id: AccountId {
                    iban: "debtor_iban".to_string(),
                },
                currency: "EUR".to_string(),
            },
            debtor_agent: DebtorAgent {
                financial_institution_id: FinancialInstitutionId {
                    bic: "debtor_bic".to_string(),
                },
            },
            credit_transfer_transaction_info: vec![CreditTransferTransactionInfo {
                payment_id: PaymentId {
                    end_to_end_id: "end_to_end_id".to_string(),
                },
                amount: Amount {
                    instructed_amount: InstructedAmount {
                        currency: "EUR".to_string(),
                        value: "100".to_string(),
                    },
                },
                creditor_agent: CreditorAgent {
                    financial_institution_id: FinancialInstitutionId {
                        bic: "creditor_agent_bic".to_string(),
                    },
                },
                creditor: Creditor {
                    name: "creditor_name".to_string(),
                    postal_address: PostalAddress {
                        country: "creditor_country".to_string(),
                        address_line: vec!["creditor_address_line_1".to_string()],
                    },
                },
                creditor_account: CreditorAccount {
                    id: AccountId {
                        iban: "creditor_iban".to_string(),
                    },
                },
                remittance_info: RemittanceInfo {
                    unstructured: "unstructured".to_string(),
                },
            }],
        };

        assert_eq!(payment_info.payment_info_id, "payment_info_id");
        assert_eq!(payment_info.payment_method, "payment_method");
        assert_eq!(payment_info.number_of_transactions, "10");
        assert_eq!(payment_info.control_sum, "100");
        assert_eq!(payment_info.payment_type_info.service_level.code, "SEPA");
        assert_eq!(
            payment_info.payment_type_info.payment_type_id,
            "payment_type_id"
        );
        assert_eq!(payment_info.requested_execution_date, "2023-01-01");
        assert_eq!(payment_info.debtor.name, "debtor_name");
        assert_eq!(payment_info.debtor.postal_address.country, "debtor_country");
        assert_eq!(
            payment_info.debtor.postal_address.address_line[0],
            "debtor_address_line_1"
        );
        assert_eq!(payment_info.debtor_account.id.iban, "debtor_iban");
        assert_eq!(payment_info.debtor_account.currency, "EUR");
        assert_eq!(
            payment_info.debtor_agent.financial_institution_id.bic,
            "debtor_bic"
        );
        assert_eq!(
            payment_info.credit_transfer_transaction_info[0]
                .payment_id
                .end_to_end_id,
            "end_to_end_id"
        );
        assert_eq!(
            payment_info.credit_transfer_transaction_info[0]
                .amount
                .instructed_amount
                .currency,
            "EUR"
        );
        assert_eq!(
            payment_info.credit_transfer_transaction_info[0]
                .amount
                .instructed_amount
                .value,
            "100"
        );
        assert_eq!(
            payment_info.credit_transfer_transaction_info[0]
                .creditor_agent
                .financial_institution_id
                .bic,
            "creditor_agent_bic"
        );
        assert_eq!(
            payment_info.credit_transfer_transaction_info[0]
                .creditor
                .name,
            "creditor_name"
        );
        assert_eq!(
            payment_info.credit_transfer_transaction_info[0]
                .creditor
                .postal_address
                .country,
            "creditor_country"
        );
        assert_eq!(
            payment_info.credit_transfer_transaction_info[0]
                .creditor
                .postal_address
                .address_line[0],
            "creditor_address_line_1"
        );
        assert_eq!(
            payment_info.credit_transfer_transaction_info[0]
                .creditor_account
                .id
                .iban,
            "creditor_iban"
        );
        assert_eq!(
            payment_info.credit_transfer_transaction_info[0]
                .payment_id
                .end_to_end_id,
            "end_to_end_id"
        );
    }
}
