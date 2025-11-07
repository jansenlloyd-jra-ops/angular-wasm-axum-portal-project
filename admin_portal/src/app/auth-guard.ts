import { ActivatedRouteSnapshot, CanActivateFn, RouterStateSnapshot } from '@angular/router';
import { Router } from '@angular/router';
import { inject } from '@angular/core';
import { request_authenticate } from '../assets/wasm_backend/wasm_backend.js'

export const authGuard: CanActivateFn = async (route: ActivatedRouteSnapshot, state: RouterStateSnapshot) => {
  const router = inject(Router);
  try {
    const authCheck: any = JSON.parse(await request_authenticate());
    console.log(authCheck);
    if (authCheck.authorized) {
      return authCheck.authorized;
    } else {
      router.navigate(['/']);
      window.alert("Unauthorized Access");
      return false;
    }
  } catch (err) {
    router.navigate(['/']);
    window.alert("Unauthorized Access");
    return false;
  }
};
