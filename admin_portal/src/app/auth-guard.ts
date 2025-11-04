import { ActivatedRouteSnapshot, CanActivateFn, RouterStateSnapshot } from '@angular/router';
import { Router } from '@angular/router';
import { inject } from '@angular/core';
import { is_authenticated } from '../assets/wasm_backend/wasm_backend.js'

export const authGuard: CanActivateFn = async (route: ActivatedRouteSnapshot, state: RouterStateSnapshot) => {
  const router = inject(Router);
  try {
    const authCheck: any = JSON.parse(await is_authenticated()); // <- request from server if there is an authenticated access await sign request
    console.log(authCheck);
    if (authCheck) {
      return authCheck.authorized;
    } else {
      router.navigate(['']);
      return false;
    }
  } catch (err) {
    router.navigate(['']);
    return false;
  }
};
