import { ActivatedRouteSnapshot, CanActivateFn, RouterStateSnapshot } from '@angular/router';
import { Router } from '@angular/router';
import { inject } from '@angular/core';

export const authGuard: CanActivateFn = (route: ActivatedRouteSnapshot, state: RouterStateSnapshot) => {
  const router = inject(Router);
  try{
    const authCheck = true ; // <- request from server if there is a authenticated access await sign request
    if (authCheck){
      return true
    }
    return false
    router.navigate(['']);
  }catch(err){
    router.navigate(['']);
    return false;
  }

};
