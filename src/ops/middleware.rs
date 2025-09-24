// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
use std::{
    future::{ready, Future},
    pin::Pin,
};

use actix_web::{
    body::{BoxBody, EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    HttpResponse,
};
use log::{debug, info};

use crate::{error::ServerError, models::connection_info::ConnectionInfo};

/// Middleware factory to verify that a client IP is present in the SAN list in a client certificate.
pub struct PeerSANVerifier;

impl<S, B> Transform<S, ServiceRequest> for PeerSANVerifier
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<B>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = PeerSANVerifierMiddleware<S>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PeerSANVerifierMiddleware { service }))
    }
}

/// Middleware that verifies the client certificate matches the origin IP of the request.
pub struct PeerSANVerifierMiddleware<S> {
    service: S,
}

impl<S> PeerSANVerifierMiddleware<S> {
    fn get_peer_cert_data_from_request(
        request: &ServiceRequest,
    ) -> Result<&ConnectionInfo, ServerError> {
        let peer_cert_data_res = request
            .conn_data::<Result<ConnectionInfo, ServerError>>()
            .ok_or(ServerError::MissingData(
                "No peer cert data was created for this connection.",
            ))?;

        peer_cert_data_res.as_ref().map_err(|e| e.clone())
    }

    fn verify_peer_ip_against_sans(
        request: &ServiceRequest,
    ) -> Result<(), ServerError> {
        let peer_cert_data = Self::get_peer_cert_data_from_request(request)?;

        debug!(
            "Retrieved peer cert data. SANs: {}",
            peer_cert_data.sans.join(", ")
        );

        let addr = request
            .peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_default();

        if !peer_cert_data.sans.contains(&addr) {
            return Err(ServerError::VerificationFailed(format!(
                "No subject alternative name matches client IP ({addr})."
            )));
        }

        Ok(())
    }
}

/// Middleware implementation relies on returning an [EitherBody] response.
/// This follows a [Result]-type pattern where the left body type is the "Ok" outcome where the client
/// passed verification by the middleware and lets the request be serviced.
/// The right body type is the "Error" outcome where the client fails to be verified and thus a response
/// directly from the middleware is supplied.
impl<S, B> Service<ServiceRequest> for PeerSANVerifierMiddleware<S>
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<B>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = actix_web::Error;
    type Future = Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>,
    >;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        debug!("Entered PeerSANVerifierMiddleware");

        info!("Verifying client IP using SANs.");

        if let Err(e) = Self::verify_peer_ip_against_sans(&req) {
            return Box::pin(async move {
                let res = req.into_response(HttpResponse::Forbidden().body(format!(
                    "Could not obtain your certificate data. Can't verify your authenticity. Error: {e}"
                )).map_into_right_body());

                Ok(res)
            });
        }

        info!("Peer IP is valid. Servicing request.");

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?.map_into_left_body();

            debug!("Leaving PeerSANVerifierMiddleware");

            Ok(res)
        })
    }
}
