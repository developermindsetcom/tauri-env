import { error, json } from '@sveltejs/kit';
import * as privateEnv from '$env/static/private';
import type { RequestHandler } from './$types';

export const prerender = true;

// TODO: not secure, make sure to limit this only to variables you have to use!
// TODO: works only in dev mode, because works only with `@sveltejs/adapter-static`
export const GET: RequestHandler = async (params) => {
    if(!prerender){
        // const { name } = await params.request.json(); // for POST
        const name = params.url.searchParams.get('name')
        
        const variable = name ? (privateEnv as any)[name] : '';
        return json({variable})
    }

    return json({variable: ''})
};